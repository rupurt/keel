//! Navigator pattern detection and draft seeding helpers.
//!
//! This module intentionally stays crate-local and deterministic: all ranking and
//! ordering decisions are pure and stable.
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Duration, Utc};

const DEFAULT_WINDOW_DAYS: i64 = 7;
const MIN_REFLECTION_COUNT: usize = 3;
const MIN_TREND_DELTA: f64 = 0.10;
const MIN_CONFIDENCE: f64 = 0.70;
const MIN_EVIDENCE_REFS: usize = 2;
const ROLLOUT_EVIDENCE_STAGES: [&str; 3] = ["startup", "continuation", "completion"];

/// Normalized reflection signal for a single candidate key.
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectionSignal {
    /// Optional context identifier.
    pub context_id: Option<String>,
    /// Optional focus area.
    pub focus_area: Option<String>,
    /// Numeric score used for trend computation.
    pub score: f64,
    /// Confidence for this signal.
    pub confidence: f64,
    /// Timestamp for trend-window filtering.
    pub observed_at: DateTime<Utc>,
    /// Evidence artifact identifier.
    pub evidence_id: String,
}

/// Derived key used to aggregate signals.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum PatternKind {
    Context,
    Focus,
}

impl PatternKind {
    fn prefix(self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::Focus => "focus",
        }
    }
}

/// Deterministic detection output for a rising pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct RisingPattern {
    pattern_id: String,
    context_id: Option<String>,
    focus_area: Option<String>,
    trend_delta: f64,
    confidence: f64,
    evidence_ids: Vec<String>,
    rank: Option<usize>,
}

impl RisingPattern {
    /// Return the normalized pattern identifier.
    pub fn pattern_id(&self) -> &str {
        &self.pattern_id
    }

    /// Return the context identifier when available.
    pub fn context_id(&self) -> Option<&str> {
        self.context_id.as_deref()
    }

    /// Return the focus area when available.
    pub fn focus_area(&self) -> Option<&str> {
        self.focus_area.as_deref()
    }

    /// Return normalized trend delta over the selected window.
    pub fn trend_delta(&self) -> f64 {
        self.trend_delta
    }

    /// Return confidence score used for seeding.
    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    /// Ranked output position, if assigned.
    pub fn rank(&self) -> Option<usize> {
        self.rank
    }

    /// Evidence refs that produced this signal.
    pub fn evidence_ids(&self) -> &[String] {
        &self.evidence_ids
    }
}

impl RisingPattern {
    fn new(
        pattern_id: String,
        context_id: Option<String>,
        focus_area: Option<String>,
        trend_delta: f64,
        confidence: f64,
        evidence_ids: Vec<String>,
    ) -> Self {
        Self {
            pattern_id,
            context_id,
            focus_area,
            trend_delta,
            confidence,
            evidence_ids,
            rank: None,
        }
    }

    fn with_rank(mut self, rank: usize) -> Self {
        self.rank = Some(rank);
        self
    }
}

/// Explicit reason when a candidate is blocked from seeding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedBlocker {
    /// Candidate identifier.
    pub pattern_id: String,
    /// Human-readable blocker explanation.
    pub reason: String,
}

/// Deterministic draft candidate intended for planner handoff.
#[derive(Debug, Clone, PartialEq)]
pub struct BearingDraft {
    pub bearing_draft_id: String,
    pub source_pattern_id: String,
    pub seeded_from_refs: Vec<String>,
    pub candidate_scope: String,
    pub candidate_title: String,
    pub rollout_phase: RolloutPhase,
    pub evidence_chain: Vec<String>,
    pub trend_delta: f64,
    pub confidence: f64,
}

/// Rollout phase for candidate sequencing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RolloutPhase {
    Canary,
    Pilot,
    Steady,
}

impl std::fmt::Display for RolloutPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Canary => write!(f, "canary"),
            Self::Pilot => write!(f, "pilot"),
            Self::Steady => write!(f, "steady"),
        }
    }
}

/// Rollout grouping of ranked drafts.
#[derive(Debug, Clone)]
pub struct RolloutBatch {
    pub phase: RolloutPhase,
    pub drafts: Vec<BearingDraft>,
}

/// Configuration for rising-pattern detection thresholds.
#[derive(Debug, Clone)]
pub struct DetectionConfig {
    /// Lookback window in days.
    pub window_days: i64,
    /// Minimum required observations per pattern.
    pub min_reflections: usize,
    /// Minimum trend delta (current - baseline / baseline).
    pub min_trend_delta: f64,
    /// Minimum confidence average.
    pub min_confidence: f64,
    /// Minimum evidence refs per rising pattern.
    pub min_evidence_refs: usize,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            window_days: DEFAULT_WINDOW_DAYS,
            min_reflections: MIN_REFLECTION_COUNT,
            min_trend_delta: MIN_TREND_DELTA,
            min_confidence: MIN_CONFIDENCE,
            min_evidence_refs: MIN_EVIDENCE_REFS,
        }
    }
}

/// Detect eligible rising patterns from normalized signals.
pub fn detect_rising_patterns(
    signals: &[ReflectionSignal],
    now: DateTime<Utc>,
    config: &DetectionConfig,
) -> Vec<RisingPattern> {
    let mut by_key: HashMap<(PatternKind, String), Vec<ReflectionSignal>> = HashMap::new();

    for signal in signals {
        for (kind, id) in pattern_keys(signal) {
            by_key.entry((kind, id)).or_default().push(signal.clone());
        }
    }

    let mut candidates = Vec::new();
    let cutoff = now - Duration::days(config.window_days);
    for ((kind, id), mut group) in by_key {
        group.retain(|signal| signal.observed_at >= cutoff);
        if group.len() < config.min_reflections {
            continue;
        }

        group.sort_by_key(|signal| signal.observed_at);

        let confidence = average_confidence(&group);
        if confidence < config.min_confidence {
            continue;
        }

        let (context_id, focus_area) = match kind {
            PatternKind::Context => (Some(id.clone()), None),
            PatternKind::Focus => (None, Some(id.clone())),
        };

        let trend_delta = compute_trend_delta(&group);
        if trend_delta < config.min_trend_delta {
            continue;
        }

        let mut evidence_ids = group
            .iter()
            .map(|signal| signal.evidence_id.clone())
            .collect::<Vec<_>>();
        evidence_ids.sort();
        evidence_ids.dedup();
        if evidence_ids.len() < config.min_evidence_refs {
            continue;
        }

        candidates.push(RisingPattern::new(
            format!("{}:{}", kind.prefix(), id),
            context_id,
            focus_area,
            trend_delta,
            confidence,
            evidence_ids,
        ));
    }

    rank_seed_candidates(candidates)
}

fn average_confidence(signals: &[ReflectionSignal]) -> f64 {
    let total = signals.iter().map(|signal| signal.confidence).sum::<f64>();
    if signals.is_empty() {
        0.0
    } else {
        total / signals.len() as f64
    }
}

fn compute_trend_delta(signals: &[ReflectionSignal]) -> f64 {
    if signals.len() < 2 {
        return 0.0;
    }

    let half = (signals.len() / 2).max(1);
    let baseline_slice = &signals[..half];
    let current_slice = &signals[half..];

    let baseline_avg = baseline_slice
        .iter()
        .map(|signal| signal.score)
        .sum::<f64>()
        / baseline_slice.len() as f64;
    let current_avg =
        current_slice.iter().map(|signal| signal.score).sum::<f64>() / current_slice.len() as f64;

    if baseline_avg <= 0.0 {
        0.0
    } else {
        (current_avg - baseline_avg) / baseline_avg
    }
}

fn pattern_keys(signal: &ReflectionSignal) -> Vec<(PatternKind, String)> {
    let mut keys = Vec::new();
    if let Some(context_id) = &signal.context_id {
        keys.push((PatternKind::Context, context_id.clone()));
    }
    if let Some(focus_area) = &signal.focus_area {
        keys.push((PatternKind::Focus, focus_area.clone()));
    }
    keys
}

/// Return candidates in deterministic downstream order.
pub fn rank_seed_candidates(mut candidates: Vec<RisingPattern>) -> Vec<RisingPattern> {
    candidates.sort_by(|a, b| {
        b.trend_delta
            .partial_cmp(&a.trend_delta)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                b.confidence
                    .partial_cmp(&a.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| a.pattern_id.cmp(&b.pattern_id))
    });

    candidates
        .into_iter()
        .enumerate()
        .map(|(rank, pattern)| pattern.with_rank(rank))
        .collect()
}

/// Compute blocker reasons for a candidate before seeding.
pub fn seed_blockers(
    candidate: &RisingPattern,
    existing_pattern_ids: &HashSet<String>,
    max_evidence_refs: usize,
) -> Vec<SeedBlocker> {
    let mut blockers = Vec::new();

    if candidate.pattern_id().trim().is_empty() {
        blockers.push(SeedBlocker {
            pattern_id: candidate.pattern_id().to_string(),
            reason: "malformed candidate identifier".to_string(),
        });
    }
    if candidate.context_id.is_none() && candidate.focus_area.is_none() {
        blockers.push(SeedBlocker {
            pattern_id: candidate.pattern_id.clone(),
            reason: "missing context_id and focus_area".to_string(),
        });
    }
    if candidate.evidence_ids().len() < max_evidence_refs {
        blockers.push(SeedBlocker {
            pattern_id: candidate.pattern_id.clone(),
            reason: format!(
                "insufficient evidence refs (found {}, required {})",
                candidate.evidence_ids().len(),
                max_evidence_refs
            ),
        });
    }
    if existing_pattern_ids.contains(&candidate.pattern_id) {
        blockers.push(SeedBlocker {
            pattern_id: candidate.pattern_id.clone(),
            reason: "duplicate pattern id detected".to_string(),
        });
    }

    blockers
}

/// Build a deterministic draft object for a rising pattern.
pub fn bearing_draft(
    candidate: &RisingPattern,
    candidate_scope: &str,
    phase: RolloutPhase,
) -> BearingDraft {
    let mut evidence_chain = candidate.evidence_ids().to_vec();
    append_rollout_evidence_chain(
        &mut evidence_chain,
        candidate,
        candidate_scope,
        phase,
        &ROLLOUT_EVIDENCE_STAGES,
    );

    let candidate_title = format!(
        "Seeded candidate for {} ({} / {:.2})",
        candidate.pattern_id(),
        phase,
        candidate.confidence()
    );

    BearingDraft {
        bearing_draft_id: format!(
            "bearing:{}:{}",
            candidate.pattern_id().replace(':', "_"),
            candidate.rank().unwrap_or(0)
        ),
        source_pattern_id: candidate.pattern_id().to_string(),
        seeded_from_refs: candidate.evidence_ids().to_vec(),
        candidate_scope: candidate_scope.to_string(),
        candidate_title,
        rollout_phase: phase,
        evidence_chain,
        trend_delta: candidate.trend_delta(),
        confidence: candidate.confidence(),
    }
}

fn append_rollout_evidence_chain(
    chain: &mut Vec<String>,
    candidate: &RisingPattern,
    candidate_scope: &str,
    phase: RolloutPhase,
    stages: &[&str],
) {
    let rank = candidate.rank().unwrap_or(0);
    for stage in stages {
        chain.push(format!(
            "evidence:{}:{}:{}:{}:{}",
            rank,
            phase,
            candidate_scope,
            candidate.pattern_id(),
            stage
        ));
    }
}

/// Build deterministic rollout batches for candidate seeding.
pub fn rollout_batches(
    candidates: &[RisingPattern],
    canary: usize,
    pilot: usize,
) -> Vec<RolloutBatch> {
    let mut used_pattern_ids = HashSet::new();
    let mut remaining = Vec::new();
    let mut batches = Vec::new();

    for candidate in candidates {
        if !used_pattern_ids.insert(candidate.pattern_id().to_string()) {
            continue;
        }
        remaining.push(candidate.clone());
    }

    let mut cursor = 0;
    let first = remaining.len().min(canary);
    if first > 0 {
        let drafts = remaining[cursor..cursor + first]
            .iter()
            .map(|candidate| bearing_draft(candidate, "unscoped", RolloutPhase::Canary))
            .collect::<Vec<_>>();
        batches.push(RolloutBatch {
            phase: RolloutPhase::Canary,
            drafts,
        });
        cursor += first;
    }

    let second = remaining.len().saturating_sub(cursor).min(pilot);
    if second > 0 {
        let drafts = remaining[cursor..cursor + second]
            .iter()
            .map(|candidate| bearing_draft(candidate, "unscoped", RolloutPhase::Pilot))
            .collect::<Vec<_>>();
        batches.push(RolloutBatch {
            phase: RolloutPhase::Pilot,
            drafts,
        });
        cursor += second;
    }

    if cursor < remaining.len() {
        let drafts = remaining[cursor..]
            .iter()
            .map(|candidate| bearing_draft(candidate, "unscoped", RolloutPhase::Steady))
            .collect::<Vec<_>>();
        batches.push(RolloutBatch {
            phase: RolloutPhase::Steady,
            drafts,
        });
    }

    batches
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-02-14T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    fn signal(
        context_id: Option<&str>,
        focus_area: Option<&str>,
        score: f64,
        confidence: f64,
        days_ago: i64,
        evidence_id: &str,
    ) -> ReflectionSignal {
        ReflectionSignal {
            context_id: context_id.map(ToString::to_string),
            focus_area: focus_area.map(ToString::to_string),
            score,
            confidence,
            observed_at: now() - Duration::days(days_ago),
            evidence_id: evidence_id.to_string(),
        }
    }

    #[test]
    fn detects_rising_context_id() {
        let now = now();
        let signals = vec![
            signal(Some("ctx-a"), None, 10.0, 0.8, 7, "ev-1"),
            signal(Some("ctx-a"), None, 11.5, 0.8, 4, "ev-2"),
            signal(Some("ctx-a"), None, 13.0, 0.9, 1, "ev-3"),
        ];

        let result = detect_rising_patterns(&signals, now, &DetectionConfig::default());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].context_id(), Some("ctx-a"));
        assert!(result[0].trend_delta() > 0.1);
    }

    #[test]
    fn ranks_deterministically_by_delta_then_confidence_then_pattern() {
        let a = RisingPattern::new(
            "b".into(),
            Some("a".into()),
            None,
            0.2,
            0.7,
            vec!["1".into(), "2".into()],
        );
        let b = RisingPattern::new(
            "a".into(),
            Some("a".into()),
            None,
            0.2,
            0.8,
            vec!["1".into(), "2".into()],
        );
        let c = RisingPattern::new(
            "c".into(),
            Some("a".into()),
            None,
            0.3,
            0.9,
            vec!["1".into(), "2".into()],
        );

        // enforce internal fields are sorted deterministically
        let ranked = rank_seed_candidates(vec![a.clone(), b.clone(), c.clone()]);
        assert_eq!(ranked[0].pattern_id(), "c");
        assert_eq!(ranked[1].pattern_id(), "a");
        assert_eq!(ranked[2].pattern_id(), "b");
        assert_eq!(ranked[0].rank(), Some(0));
        assert_eq!(ranked[1].rank(), Some(1));
    }

    #[test]
    fn blocks_duplicates_and_low_evidence() {
        let signals = vec![
            signal(Some("ctx-a"), None, 10.0, 0.8, 6, "ev-1"),
            signal(Some("ctx-a"), None, 11.0, 0.9, 3, "ev-2"),
            signal(Some("ctx-a"), None, 12.0, 0.8, 1, "ev-3"),
        ];

        let candidates = detect_rising_patterns(&signals, now(), &DetectionConfig::default());
        assert_eq!(candidates.len(), 1);
        let mut seen = HashSet::new();
        seen.insert(candidates[0].pattern_id().to_string());
        let blocker = seed_blockers(&candidates[0], &seen, MIN_EVIDENCE_REFS);
        assert_eq!(blocker.len(), 1);
        assert_eq!(blocker[0].reason, "duplicate pattern id detected");
    }

    #[test]
    fn blocks_malformed_pattern_id() {
        let candidate = RisingPattern::new(
            "".into(),
            Some("ctx".into()),
            None,
            0.2,
            0.8,
            vec!["e1".into(), "e2".into()],
        );
        let blockers = seed_blockers(&candidate, &HashSet::new(), MIN_EVIDENCE_REFS);
        assert_eq!(blockers.len(), 1);
        assert_eq!(blockers[0].reason, "malformed candidate identifier");
    }

    #[test]
    fn creates_rollout_batches() {
        let mut candidates = Vec::new();
        for idx in 0..8 {
            candidates.push(RisingPattern::new(
                format!("pattern-{idx}"),
                Some("ctx".into()),
                None,
                0.11 + idx as f64 * 0.01,
                0.75,
                vec!["e1".into(), "e2".into()],
            ));
        }
        let ranked = rank_seed_candidates(candidates);
        let batches = rollout_batches(&ranked, 1, 5);

        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0].phase, RolloutPhase::Canary);
        assert_eq!(batches[0].drafts.len(), 1);
        assert_eq!(batches[1].phase, RolloutPhase::Pilot);
        assert_eq!(batches[1].drafts.len(), 5);
        assert_eq!(batches[2].phase, RolloutPhase::Steady);
        assert_eq!(batches[2].drafts.len(), 2);
        assert_eq!(batches[2].drafts[1].evidence_chain.len(), 5);
        assert_eq!(batches[0].drafts[0].rollout_phase, RolloutPhase::Canary);
        assert!(
            batches[1].drafts[0]
                .evidence_chain
                .iter()
                .any(|entry| entry.ends_with(":startup"))
        );
        assert!(
            batches[1].drafts[0]
                .evidence_chain
                .iter()
                .any(|entry| entry.ends_with(":continuation"))
        );
        assert!(
            batches[1].drafts[0]
                .evidence_chain
                .iter()
                .any(|entry| entry.ends_with(":completion"))
        );
    }

    #[test]
    fn bearing_draft_emits_complete_rollout_evidence_stages() {
        let candidate = RisingPattern::new(
            "context:ctx-a".into(),
            Some("ctx-a".into()),
            None,
            0.22,
            0.81,
            vec!["a".into(), "b".into(), "c".into()],
        );
        let draft = bearing_draft(&candidate, "ctx-a", RolloutPhase::Pilot);

        assert_eq!(draft.evidence_chain.len(), 6);
        assert!(
            draft.evidence_chain[2..]
                .iter()
                .any(|entry| entry.ends_with(":startup"))
        );
        assert!(
            draft.evidence_chain[2..]
                .iter()
                .any(|entry| entry.ends_with(":continuation"))
        );
        assert!(
            draft.evidence_chain[2..]
                .iter()
                .any(|entry| entry.ends_with(":completion"))
        );
        assert!(
            draft
                .evidence_chain
                .iter()
                .any(|entry| entry.contains("pilot"))
        );
    }
}
