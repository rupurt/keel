//! Conservative pairwise conflict scoring for `next --parallel`.

use std::cmp::Ordering;

use crate::cli::commands::management::next_support::parallel_features::ParallelFeatureVector;
use crate::domain::policy::queue::compare_work_item_ids;

/// Pairwise score used by the parallel conflict gate.
#[derive(Debug, Clone, PartialEq)]
pub struct PairwiseConflictScore {
    pub story_id: String,
    pub blocked_by_story_id: String,
    /// Conflict risk in the closed range [0.0, 1.0].
    pub risk: f64,
    /// Scoring confidence in the closed range [0.0, 1.0].
    pub confidence: f64,
}

/// Score every pairwise feature vector with a conservative risk posture.
pub fn score_parallel_pairwise_conflicts(
    feature_vectors: &[ParallelFeatureVector],
) -> Vec<PairwiseConflictScore> {
    let mut scores: Vec<_> = feature_vectors.iter().map(score_vector).collect();
    scores.sort_by(stable_pair_order);
    scores
}

fn score_vector(vector: &ParallelFeatureVector) -> PairwiseConflictScore {
    let mut risk = 0.05;
    let mut confidence = 0.95;

    if vector.same_scope {
        risk += 0.35;
        confidence -= 0.05;
    }
    if vector.same_voyage {
        risk += 0.20;
        confidence -= 0.04;
    }
    if vector.same_epic {
        risk += 0.08;
        confidence -= 0.02;
    }
    if vector.same_story_type {
        risk += 0.06;
    }
    if vector.overlapping_roles {
        risk += 0.08;
    }
    risk += (vector.shared_governance_adrs.len().min(3) as f64) * 0.06;

    if !vector.unresolved_context.is_empty() {
        let unresolved_count = vector.unresolved_context.len() as f64;
        risk += 0.30 * unresolved_count;
        confidence -= 0.25 * unresolved_count;

        // Conservative fallback posture when semantics are unresolved.
        risk = risk.max(0.75);
        confidence = confidence.min(0.50);
    }

    PairwiseConflictScore {
        story_id: vector.story_id.clone(),
        blocked_by_story_id: vector.blocked_by_story_id.clone(),
        risk: risk.clamp(0.0, 1.0),
        confidence: confidence.clamp(0.0, 1.0),
    }
}

fn stable_pair_order(left: &PairwiseConflictScore, right: &PairwiseConflictScore) -> Ordering {
    compare_work_item_ids(&left.story_id, &right.story_id)
        .then_with(|| compare_work_item_ids(&left.blocked_by_story_id, &right.blocked_by_story_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::commands::management::next_support::parallel_features::extract_parallel_feature_vectors;
    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    fn score_for_pair<'a>(
        scores: &'a [PairwiseConflictScore],
        story_id: &str,
        blocked_by_story_id: &str,
    ) -> &'a PairwiseConflictScore {
        scores
            .iter()
            .find(|score| {
                score.story_id == story_id && score.blocked_by_story_id == blocked_by_story_id
            })
            .unwrap()
    }

    #[test]
    fn next_parallel_pairwise_scoring_is_conservative() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .title("Core auth work")
                    .scope("core/01-auth")
                    .role("engineer/software")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S2")
                    .title("Core storage work")
                    .scope("core/02-storage")
                    .role("engineer/software")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S3")
                    .title("Ops pipeline work")
                    .scope("ops/01-ci")
                    .role("operator/infra")
                    .stage(StoryState::Backlog),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let s1 = board.stories.get("S1").unwrap();
        let s2 = board.stories.get("S2").unwrap();
        let s3 = board.stories.get("S3").unwrap();

        let vectors = extract_parallel_feature_vectors(&board, &[s3, s1, s2]);
        let scores = score_parallel_pairwise_conflicts(&vectors);

        assert_eq!(scores.len(), vectors.len());
        for score in &scores {
            assert!((0.0..=1.0).contains(&score.risk));
            assert!((0.0..=1.0).contains(&score.confidence));
        }

        let vectors_second_pass = extract_parallel_feature_vectors(&board, &[s2, s3, s1]);
        let scores_second_pass = score_parallel_pairwise_conflicts(&vectors_second_pass);
        assert_eq!(scores, scores_second_pass);

        let core_pair = score_for_pair(&scores, "S1", "S2");
        let cross_epic_pair = score_for_pair(&scores, "S1", "S3");
        assert!(
            core_pair.risk > cross_epic_pair.risk,
            "expected core-core pair risk ({}) > cross-epic risk ({})",
            core_pair.risk,
            cross_epic_pair.risk
        );
    }

    #[test]
    fn next_parallel_pairwise_scoring_penalizes_uncertainty() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S-CERTAIN-A")
                    .title("Certain A")
                    .scope("core/01-a")
                    .role("engineer/software")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S-CERTAIN-B")
                    .title("Certain B")
                    .scope("core/02-b")
                    .role("engineer/software")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S-UNKNOWN")
                    .title("Unknown structure")
                    .stage(StoryState::Backlog),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let certain_a = board.stories.get("S-CERTAIN-A").unwrap();
        let certain_b = board.stories.get("S-CERTAIN-B").unwrap();
        let unknown = board.stories.get("S-UNKNOWN").unwrap();

        let certain_vectors = extract_parallel_feature_vectors(&board, &[certain_a, certain_b]);
        let uncertain_vectors = extract_parallel_feature_vectors(&board, &[unknown, certain_a]);

        let certain_score = score_parallel_pairwise_conflicts(&certain_vectors)
            .into_iter()
            .next()
            .unwrap();
        let uncertain_score = score_parallel_pairwise_conflicts(&uncertain_vectors)
            .into_iter()
            .next()
            .unwrap();

        assert!(
            uncertain_score.risk > certain_score.risk,
            "unknown context should increase risk: uncertain={} certain={}",
            uncertain_score.risk,
            certain_score.risk
        );
        assert!(
            uncertain_score.confidence < certain_score.confidence,
            "unknown context should reduce confidence: uncertain={} certain={}",
            uncertain_score.confidence,
            certain_score.confidence
        );
        assert!(uncertain_score.risk >= 0.75);
        assert!(uncertain_score.confidence <= 0.50);
    }
}
