//! Confidence-threshold gate for parallel candidate selection.

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::cli::commands::management::next_support::parallel_scoring::PairwiseConflictScore;
use crate::domain::model::Story;
use crate::domain::policy::queue::compare_work_item_ids;

/// Global confidence threshold for parallel eligibility.
pub const PARALLEL_CONFIDENCE_THRESHOLD: f64 = 0.70;

/// Pairwise blocker emitted when a candidate fails confidence gating.
#[derive(Debug, Clone, PartialEq)]
pub struct PairwiseBlocker {
    pub story_id: String,
    pub blocked_by_story_id: String,
    pub reasons: Vec<String>,
    pub confidence: f64,
}

/// Result of threshold-based parallel candidate selection.
#[derive(Debug, Clone)]
pub struct ParallelThresholdSelection<'a> {
    pub selected: Vec<&'a Story>,
    pub blocked_pairs: Vec<PairwiseBlocker>,
}

/// Select a deterministic subset of candidates whose pairwise confidence
/// meets the global threshold.
pub fn select_parallel_candidates_with_confidence_threshold<'a>(
    ready: &[&'a Story],
    pairwise_scores: &[PairwiseConflictScore],
) -> ParallelThresholdSelection<'a> {
    let mut ordered_candidates = ready.to_vec();
    ordered_candidates.sort_by(|left, right| compare_work_item_ids(left.id(), right.id()));

    let mut confidence_by_pair = HashMap::new();
    for score in pairwise_scores {
        confidence_by_pair.insert(
            canonical_pair_key(&score.story_id, &score.blocked_by_story_id),
            score.confidence,
        );
    }

    let mut selected: Vec<&'a Story> = Vec::new();
    let mut blocked_pairs = Vec::new();
    for candidate in ordered_candidates {
        let mut candidate_blockers = Vec::new();
        for selected_story in &selected {
            let pair_key = canonical_pair_key(candidate.id(), selected_story.id());
            let pair_confidence = confidence_by_pair.get(&pair_key).copied().unwrap_or(0.0);
            if blocked_by_override(candidate, selected_story) {
                candidate_blockers.push(PairwiseBlocker {
                    story_id: candidate.id().to_string(),
                    blocked_by_story_id: selected_story.id().to_string(),
                    reasons: vec![format!(
                        "metadata override: blocked_by constraint between {} and {}",
                        candidate.id(),
                        selected_story.id()
                    )],
                    confidence: pair_confidence,
                });
            } else if pair_confidence < PARALLEL_CONFIDENCE_THRESHOLD {
                candidate_blockers.push(PairwiseBlocker {
                    story_id: candidate.id().to_string(),
                    blocked_by_story_id: selected_story.id().to_string(),
                    reasons: vec![format!(
                        "confidence {:.2} below threshold {:.2}",
                        pair_confidence, PARALLEL_CONFIDENCE_THRESHOLD
                    )],
                    confidence: pair_confidence,
                });
            }
        }

        if candidate_blockers.is_empty() {
            selected.push(candidate);
        } else {
            blocked_pairs.extend(candidate_blockers);
        }
    }

    blocked_pairs.sort_by(stable_blocker_order);
    ParallelThresholdSelection {
        selected,
        blocked_pairs,
    }
}

fn canonical_pair_key(left: &str, right: &str) -> (String, String) {
    match compare_work_item_ids(left, right) {
        Ordering::Greater => (right.to_string(), left.to_string()),
        _ => (left.to_string(), right.to_string()),
    }
}

fn blocked_by_override(left: &Story, right: &Story) -> bool {
    left.frontmatter
        .blocked_by
        .iter()
        .any(|story_id| story_id == right.id())
        || right
            .frontmatter
            .blocked_by
            .iter()
            .any(|story_id| story_id == left.id())
}

fn stable_blocker_order(left: &PairwiseBlocker, right: &PairwiseBlocker) -> Ordering {
    compare_work_item_ids(&left.story_id, &right.story_id)
        .then_with(|| compare_work_item_ids(&left.blocked_by_story_id, &right.blocked_by_story_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::commands::management::next_support::parallel_features::extract_parallel_feature_vectors;
    use crate::cli::commands::management::next_support::parallel_scoring::score_parallel_pairwise_conflicts;
    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn next_parallel_threshold_blocks_uncertain_pairs() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .title("Structured core work")
                    .scope("core/01-structured")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S2")
                    .title("Unknown architecture work")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S3")
                    .title("Structured ops work")
                    .scope("ops/01-structured")
                    .stage(StoryState::Backlog),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let s1 = board.stories.get("S1").unwrap();
        let s2 = board.stories.get("S2").unwrap();
        let s3 = board.stories.get("S3").unwrap();

        let ready = vec![s3, s2, s1];
        let feature_vectors = extract_parallel_feature_vectors(&board, &ready);
        let pairwise_scores = score_parallel_pairwise_conflicts(&feature_vectors);

        let unknown_pair = pairwise_scores
            .iter()
            .find(|score| score.story_id == "S1" && score.blocked_by_story_id == "S2")
            .unwrap();
        assert!(unknown_pair.confidence < PARALLEL_CONFIDENCE_THRESHOLD);

        let selection =
            select_parallel_candidates_with_confidence_threshold(&ready, &pairwise_scores);
        let selected_ids: Vec<_> = selection.selected.iter().map(|story| story.id()).collect();

        assert_eq!(selected_ids, vec!["S1", "S3"]);
        assert!(
            !selected_ids.contains(&"S2"),
            "uncertain story should be conservatively excluded"
        );
        assert_eq!(selection.blocked_pairs.len(), 1);
        assert_eq!(selection.blocked_pairs[0].story_id, "S2");
        assert_eq!(selection.blocked_pairs[0].blocked_by_story_id, "S1");
        assert!(
            selection.blocked_pairs[0].reasons[0].contains("confidence 0.50 below threshold 0.70")
        );
    }

    #[test]
    fn next_parallel_blocked_by_override_enforced() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .title("Structured core work")
                    .scope("core/01-structured")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S2")
                    .title("Structured ops work")
                    .scope("ops/01-structured")
                    .blocked_by(&["S1"])
                    .stage(StoryState::Backlog),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let s1 = board.stories.get("S1").unwrap();
        let s2 = board.stories.get("S2").unwrap();

        let ready = vec![s2, s1];
        let feature_vectors = extract_parallel_feature_vectors(&board, &ready);
        let pairwise_scores = score_parallel_pairwise_conflicts(&feature_vectors);
        let selection =
            select_parallel_candidates_with_confidence_threshold(&ready, &pairwise_scores);

        let selected_ids: Vec<_> = selection.selected.iter().map(|story| story.id()).collect();
        assert_eq!(selected_ids, vec!["S1"]);
        assert_eq!(selection.blocked_pairs.len(), 1);
        assert_eq!(selection.blocked_pairs[0].story_id, "S2");
        assert_eq!(selection.blocked_pairs[0].blocked_by_story_id, "S1");
        assert!(
            selection.blocked_pairs[0].reasons[0].contains("metadata override"),
            "expected blocked_by metadata override reason"
        );
    }
}
