//! Deterministic semantic feature extraction for `next --parallel`.

use std::collections::BTreeSet;

use crate::domain::model::{Board, Story};
use crate::domain::policy::queue::compare_work_item_ids;

/// Pairwise semantic features used by parallel conflict scoring.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelFeatureVector {
    pub story_id: String,
    pub blocked_by_story_id: String,
    pub same_epic: bool,
    pub same_voyage: bool,
    pub same_scope: bool,
    pub same_story_type: bool,
    pub shared_governance_adrs: Vec<String>,
    pub overlapping_roles: bool,
    pub unresolved_context: Vec<UnresolvedContextSignal>,
}

/// Explicit unresolved-context signals for conservative downstream scoring.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnresolvedContextSignal {
    MissingScope { story_id: String },
    MissingVoyageSegment { story_id: String, scope: String },
    InvalidRoleTaxonomy { story_id: String },
}

/// Compute deterministic pairwise feature vectors from candidate stories.
pub fn extract_parallel_feature_vectors(
    _board: &Board,
    candidates: &[&Story],
) -> Vec<ParallelFeatureVector> {
    let mut ordered_candidates = candidates.to_vec();
    ordered_candidates.sort_by(|a, b| compare_work_item_ids(a.id(), b.id()));

    let mut vectors = Vec::new();
    for left_idx in 0..ordered_candidates.len() {
        for right_idx in left_idx + 1..ordered_candidates.len() {
            let left = ordered_candidates[left_idx];
            let right = ordered_candidates[right_idx];
            vectors.push(build_feature_vector(left, right));
        }
    }
    vectors
}

fn build_feature_vector(left: &Story, right: &Story) -> ParallelFeatureVector {
    ParallelFeatureVector {
        story_id: left.id().to_string(),
        blocked_by_story_id: right.id().to_string(),
        same_epic: left.epic().is_some() && left.epic() == right.epic(),
        same_voyage: left.voyage().is_some() && left.voyage() == right.voyage(),
        same_scope: left.scope().is_some() && left.scope() == right.scope(),
        same_story_type: left.story_type() == right.story_type(),
        shared_governance_adrs: shared_governance_adrs(left, right),
        overlapping_roles: roles_overlap(left, right),
        unresolved_context: unresolved_context_signals(left, right),
    }
}

fn shared_governance_adrs(left: &Story, right: &Story) -> Vec<String> {
    let left_set: BTreeSet<&str> = left
        .frontmatter
        .governed_by
        .iter()
        .map(String::as_str)
        .collect();
    let right_set: BTreeSet<&str> = right
        .frontmatter
        .governed_by
        .iter()
        .map(String::as_str)
        .collect();

    left_set
        .intersection(&right_set)
        .map(|adr_id| (*adr_id).to_string())
        .collect()
}

fn roles_overlap(left: &Story, right: &Story) -> bool {
    match (left.required_role(), right.required_role()) {
        (Some(Ok(left_role)), Some(Ok(right_role))) => {
            left_role.role == right_role.role
                && left_role.specialization == right_role.specialization
        }
        _ => false,
    }
}

fn unresolved_context_signals(left: &Story, right: &Story) -> Vec<UnresolvedContextSignal> {
    let mut signals = BTreeSet::new();
    append_story_signals(left, &mut signals);
    append_story_signals(right, &mut signals);
    signals.into_iter().collect()
}

fn append_story_signals(story: &Story, signals: &mut BTreeSet<UnresolvedContextSignal>) {
    if story.scope().is_none() {
        signals.insert(UnresolvedContextSignal::MissingScope {
            story_id: story.id().to_string(),
        });
    } else if story.voyage().is_none() {
        signals.insert(UnresolvedContextSignal::MissingVoyageSegment {
            story_id: story.id().to_string(),
            scope: story.scope().unwrap_or_default().to_string(),
        });
    }

    if matches!(story.required_role(), Some(Err(_))) {
        signals.insert(UnresolvedContextSignal::InvalidRoleTaxonomy {
            story_id: story.id().to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn next_parallel_feature_vectors_are_deterministic() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S10")
                    .title("Tenth story")
                    .scope("core/10-risk")
                    .role("engineer/software")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S2")
                    .title("Second story")
                    .scope("core/02-risk")
                    .role("engineer/software")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S1")
                    .title("First story")
                    .scope("core/01-risk")
                    .stage(StoryState::Backlog),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let s1 = board.stories.get("S1").unwrap();
        let s2 = board.stories.get("S2").unwrap();
        let s10 = board.stories.get("S10").unwrap();

        let first_pass = extract_parallel_feature_vectors(&board, &[s10, s2, s1]);
        let second_pass = extract_parallel_feature_vectors(&board, &[s1, s10, s2]);

        assert_eq!(first_pass, second_pass);
        assert_eq!(first_pass.len(), 3);

        let pair_ids: BTreeSet<_> = first_pass
            .iter()
            .map(|vector| {
                (
                    vector.story_id.as_str(),
                    vector.blocked_by_story_id.as_str(),
                )
            })
            .collect();
        assert_eq!(
            pair_ids,
            BTreeSet::from([("S1", "S10"), ("S1", "S2"), ("S10", "S2")])
        );
    }

    #[test]
    fn next_parallel_feature_vectors_emit_unknown_risk() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S-MISSING-SCOPE")
                    .title("Missing architectural scope")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("S-STRUCTURED")
                    .title("Scoped story")
                    .scope("core/01-risk")
                    .stage(StoryState::Backlog),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let missing = board.stories.get("S-MISSING-SCOPE").unwrap();
        let structured = board.stories.get("S-STRUCTURED").unwrap();

        let vectors = extract_parallel_feature_vectors(&board, &[missing, structured]);
        assert_eq!(vectors.len(), 1);

        let vector = &vectors[0];
        assert!(!vector.unresolved_context.is_empty());
        assert_eq!(vector.story_id, "S-MISSING-SCOPE");
        assert_eq!(vector.blocked_by_story_id, "S-STRUCTURED");
        assert!(
            vector
                .unresolved_context
                .contains(&UnresolvedContextSignal::MissingScope {
                    story_id: "S-MISSING-SCOPE".to_string(),
                })
        );
    }
}
