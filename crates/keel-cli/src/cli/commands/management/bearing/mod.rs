//! Bearing commands - research, assess, list, show

use std::fs;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use clap::Subcommand;

pub mod file;
pub(crate) mod guidance;
pub mod show;

const BEARING_STATUS_VALUES: &[&str] = &[
    "exploring",
    "evaluating",
    "ready",
    "laid",
    "parked",
    "declined",
];

pub(crate) fn parse_bearing_status(value: &str) -> Result<String, String> {
    crate::cli::commands::management::status_filter::validate_status_arg(
        value,
        BEARING_STATUS_VALUES,
    )
}

#[derive(Subcommand, Debug)]
pub enum BearingAction {
    /// Create a new bearing
    New {
        /// Bearing title
        name: String,
    },
    /// Advance a bearing into the research stage or capture evidence
    Research {
        /// Bearing ID
        id: String,
        /// Evidence source class (web, academic, social, manual)
        #[arg(long)]
        class: Option<String>,
        /// Provider or provenance label stored with the captured source
        #[arg(long)]
        provider: Option<String>,
        /// Source URL, path, or origin
        #[arg(long)]
        location: Option<String>,
        /// Source observation or publication date (YYYY-MM-DD)
        #[arg(long = "observed-at")]
        observed_at: Option<String>,
        /// Date the source was retrieved (YYYY-MM-DD)
        #[arg(long = "retrieved-at")]
        retrieved_at: Option<String>,
        /// Evidence authority rating (low, medium, high)
        #[arg(long)]
        authority: Option<String>,
        /// Evidence freshness rating (low, medium, high)
        #[arg(long)]
        freshness: Option<String>,
        /// Authored claim or note supported by the source
        #[arg(long)]
        notes: Option<String>,
    },
    /// Add ASSESSMENT.md to a bearing
    Assess {
        /// Bearing ID
        id: String,
    },
    /// List all bearings
    List {
        /// Filter by status. Repeat to override or use + / - to modify the default active view.
        #[arg(long, action = clap::ArgAction::Append, value_parser = parse_bearing_status)]
        status: Vec<String>,
    },
    /// Show bearing details
    Show {
        /// Bearing ID or HEAD selector (HEAD, HEAD~, HEAD~~, HEAD^)
        id: String,
    },
    /// Show a bearing markdown document
    File {
        /// Bearing ID
        id: String,
        /// Document name (README, BRIEF, EVIDENCE, ASSESSMENT)
        file: String,
        /// Print raw markdown without terminal rendering
        #[arg(long)]
        raw: bool,
    },
    /// Park a bearing for later
    Park {
        /// Bearing ID
        id: String,
    },
    /// Decline a bearing with reason
    Decline {
        /// Bearing ID
        id: String,
        /// Reason for declining
        reason: String,
    },
    /// Graduate bearing to epic
    Lay {
        /// Bearing ID
        id: String,
    },
}

pub mod new;

use crate::cli::table::Table;
use guidance::{
    BearingLifecycleAction, error_with_recovery, guidance_for_action, informational_for_list,
    print_human,
};
use keel::domain::model::{Bearing, BearingStatus, Board};
use keel::infrastructure::bearing_evidence::{EvidenceSourceClass, EvidenceStrength};
use keel::infrastructure::bearing_readiness::evaluate_bearing_readiness;
use keel::infrastructure::bearing_research::{self, ResearchCaptureRequest};
use keel::infrastructure::config::{find_board_dir, load_config};
use keel::infrastructure::frontmatter_mutation::{Mutation, apply};
use keel::infrastructure::loader::load_board;
use keel::infrastructure::markdown_sections::extract_section;
use keel::infrastructure::scoring::load_bearing_score;
use keel::infrastructure::template_rendering;
use keel::infrastructure::templates;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FogType {
    Blocking,
    Protecting,
    Clear,
}

#[derive(Debug, Clone)]
struct BearingListRow {
    id: String,
    title: String,
    status: String,
    evidence: String,
    assessment: String,
    readiness: String,
    ev: String,
}

#[derive(Debug, Clone, Default)]
struct ResearchCaptureArgs {
    class: Option<String>,
    provider: Option<String>,
    location: Option<String>,
    observed_at: Option<String>,
    retrieved_at: Option<String>,
    authority: Option<String>,
    freshness: Option<String>,
    notes: Option<String>,
}

impl ResearchCaptureArgs {
    fn into_request(self) -> Result<Option<ResearchCaptureRequest>> {
        let has_capture_args = self.class.is_some()
            || self.provider.is_some()
            || self.location.is_some()
            || self.observed_at.is_some()
            || self.retrieved_at.is_some()
            || self.authority.is_some()
            || self.freshness.is_some()
            || self.notes.is_some();

        if !has_capture_args {
            return Ok(None);
        }

        let class = EvidenceSourceClass::from_str(
            self.class
                .as_deref()
                .ok_or_else(|| anyhow!("--class is required when capturing evidence"))?,
        )
        .map_err(|err| anyhow!(err))?;
        let observed_or_published_at = chrono::NaiveDate::parse_from_str(
            self.observed_at
                .as_deref()
                .ok_or_else(|| anyhow!("--observed-at is required when capturing evidence"))?,
            "%Y-%m-%d",
        )
        .map_err(|_| anyhow!("--observed-at must use YYYY-MM-DD"))?;
        let retrieved_at = chrono::NaiveDate::parse_from_str(
            self.retrieved_at
                .as_deref()
                .ok_or_else(|| anyhow!("--retrieved-at is required when capturing evidence"))?,
            "%Y-%m-%d",
        )
        .map_err(|_| anyhow!("--retrieved-at must use YYYY-MM-DD"))?;
        let authority = EvidenceStrength::from_str(
            self.authority
                .as_deref()
                .ok_or_else(|| anyhow!("--authority is required when capturing evidence"))?,
        )
        .map_err(|err| anyhow!(err))?;
        let freshness = EvidenceStrength::from_str(
            self.freshness
                .as_deref()
                .ok_or_else(|| anyhow!("--freshness is required when capturing evidence"))?,
        )
        .map_err(|err| anyhow!(err))?;

        Ok(Some(ResearchCaptureRequest {
            class,
            provider: self.provider,
            location: self
                .location
                .ok_or_else(|| anyhow!("--location is required when capturing evidence"))?,
            observed_or_published_at,
            retrieved_at,
            authority,
            freshness,
            notes: self
                .notes
                .ok_or_else(|| anyhow!("--notes is required when capturing evidence"))?,
        }))
    }
}

impl FogType {
    fn as_banner(&self) -> &'static str {
        match self {
            Self::Blocking => "🚧 blocking fog",
            Self::Protecting => "🌫️ protecting fog",
            Self::Clear => "✨ clear skies",
        }
    }
}

/// Run the new command - creates a new bearing
pub fn run_new(name: &str) -> Result<()> {
    let bearing_id = new::run(name)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::New, name, err))?;
    let guidance = guidance_for_action(BearingLifecycleAction::New, &bearing_id);
    print_human(guidance.as_ref());
    Ok(())
}

/// Run a bearing action through the bearing interface adapter.
pub fn run(_ctx: &spoke_auth::ExecutionContext, action: BearingAction) -> Result<()> {
    match action {
        BearingAction::New { name } => run_new(&name),
        BearingAction::Research {
            id,
            class,
            provider,
            location,
            observed_at,
            retrieved_at,
            authority,
            freshness,
            notes,
        } => run_research(
            &id,
            ResearchCaptureArgs {
                class,
                provider,
                location,
                observed_at,
                retrieved_at,
                authority,
                freshness,
                notes,
            },
        ),
        BearingAction::Assess { id } => run_assess(&id),
        BearingAction::List { status } => run_list(&status),
        BearingAction::Show { id } => show::run(&id),
        BearingAction::File { id, file, raw } => {
            crate::cli::commands::management::bearing::file::run(&id, &file, raw)
        }
        BearingAction::Park { id } => run_park(&id),
        BearingAction::Decline { id, reason } => run_decline(&id, &reason),
        BearingAction::Lay { id } => run_lay(&id),
    }
}

/// Run the research command - advances a bearing into the research stage
fn run_research(pattern: &str, capture: ResearchCaptureArgs) -> Result<()> {
    run_research_impl(pattern, capture)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Research, pattern, err))
}

fn run_research_impl(pattern: &str, capture: ResearchCaptureArgs) -> Result<()> {
    use keel::domain::transitions::bearing::{bearing_transitions, execute};

    let board_dir = find_board_dir()?;
    let capture_request = capture.into_request()?;

    if let Some(capture_request) = capture_request {
        let result = run_research_capture(&board_dir, pattern, capture_request)?;
        if let Some(transition) = result.transition {
            if let Some(path) = transition.file_created {
                println!("Created: {}", path);
            }
            println!("  {} → {}", transition.from, transition.to);
        }
        println!(
            "Captured: {} [{} via {}]",
            result.captured.id, result.captured.class, result.captured.provenance
        );
        println!("  {}", result.evidence_path.display());
        let guidance = guidance_for_action(BearingLifecycleAction::Research, &result.bearing_id);
        print_human(guidance.as_ref());
        return Ok(());
    }

    let result = execute(&board_dir, pattern, &bearing_transitions::RESEARCH)?;

    if let Some(path) = &result.file_created {
        println!("Created: {}", path);
    }
    println!("  {} → {}", result.from, result.to);
    let guidance = guidance_for_action(BearingLifecycleAction::Research, &result.bearing_id);
    print_human(guidance.as_ref());

    Ok(())
}

#[derive(Debug)]
struct ResearchCaptureRunResult {
    bearing_id: String,
    evidence_path: std::path::PathBuf,
    transition: Option<ResearchTransitionSummary>,
    captured: keel::infrastructure::bearing_evidence::EvidenceRecord,
}

#[derive(Debug)]
struct ResearchTransitionSummary {
    from: BearingStatus,
    to: BearingStatus,
    file_created: Option<String>,
}

fn run_research_capture(
    board_dir: &Path,
    pattern: &str,
    request: ResearchCaptureRequest,
) -> Result<ResearchCaptureRunResult> {
    let (config, _) = load_config();
    run_research_capture_with_config(board_dir, pattern, request, &config)
}

fn run_research_capture_with_config(
    board_dir: &Path,
    pattern: &str,
    mut request: ResearchCaptureRequest,
    config: &keel::infrastructure::config::Config,
) -> Result<ResearchCaptureRunResult> {
    use keel::domain::transitions::bearing::{bearing_transitions, execute};

    let board = load_board(board_dir)?;
    let bearing = board.require_bearing(pattern)?;
    request.provider = Some(bearing_research::resolve_capture_provenance(
        config,
        request.provider.as_deref(),
        &request.class,
    )?);
    let evidence_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("EVIDENCE.md");

    let transition = match bearing.status() {
        BearingStatus::Exploring => {
            Some(execute(board_dir, pattern, &bearing_transitions::RESEARCH)?)
        }
        BearingStatus::Evaluating | BearingStatus::Ready => None,
        other => {
            return Err(anyhow!(
                "Cannot capture research evidence for bearing '{}' from '{}' state",
                bearing.id(),
                other
            ));
        }
    };

    if !evidence_path.exists() {
        return Err(anyhow!(
            "bearing '{}' is missing EVIDENCE.md; rerun `keel bearing research {}` to repair the bundle",
            bearing.id(),
            bearing.id()
        ));
    }

    let capture = bearing_research::capture_research_evidence_file(&evidence_path, &[request])?;
    let captured = capture
        .appended_records
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("research capture did not produce a source record"))?;

    Ok(ResearchCaptureRunResult {
        bearing_id: bearing.id().to_string(),
        evidence_path,
        transition: transition.map(|transition| ResearchTransitionSummary {
            from: transition.from,
            to: transition.to,
            file_created: transition.file_created,
        }),
        captured,
    })
}

/// Run the assess command - adds ASSESSMENT.md to a bearing
pub fn run_assess(pattern: &str) -> Result<()> {
    run_assess_impl(pattern)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Assess, pattern, err))
}

fn run_assess_impl(pattern: &str) -> Result<()> {
    use keel::domain::transitions::bearing::{bearing_transitions, execute};

    let board_dir = find_board_dir()?;
    let (config, _source) = load_config();
    let result = execute(&board_dir, pattern, &bearing_transitions::ASSESS)?;
    let board = load_board(&board_dir)?;
    let bearing = board.require_bearing(&result.bearing_id)?;
    let readiness =
        evaluate_bearing_readiness(&board_dir, bearing, Some(&config.current_weights()));

    if let Some(path) = &result.file_created {
        println!("Created: {}", path);
    }
    println!("  {} → {}", result.from, result.to);
    if readiness.is_decision_ready() {
        let guidance = guidance_for_action(BearingLifecycleAction::Assess, &result.bearing_id);
        print_human(guidance.as_ref());
    } else {
        println!("Decision readiness blocked:");
        for message in readiness.problem_messages(&result.bearing_id) {
            println!("  - {}", message);
        }
        if let Some(command) = readiness.primary_recovery_command(&result.bearing_id) {
            println!("\nNext step:\n  {}", command);
        }
    }

    Ok(())
}

/// Update the status in a bearing's README.md
fn update_bearing_status(
    board_dir: &std::path::Path,
    bearing: &Bearing,
    new_status: BearingStatus,
    goals: &[String],
) -> Result<()> {
    let readme_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("README.md");

    let content = fs::read_to_string(&readme_path)
        .with_context(|| format!("Failed to read README.md: {}", readme_path.display()))?;

    let mut mutations = vec![Mutation::set("status", new_status.to_string())];
    if new_status == BearingStatus::Laid {
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        mutations.push(Mutation::set("laid_at", now));
        // Persist the epic lineage token — the bearing ID is the epic ID
        mutations.push(Mutation::set("epic", bearing.id()));
        if !goals.is_empty() {
            let yaml_array = format!("[{}]", goals.join(", "));
            mutations.push(Mutation::set("goals", yaml_array));
        }
    }
    let updated = apply(&content, &mutations);

    fs::write(&readme_path, updated)
        .with_context(|| format!("Failed to write README.md: {}", readme_path.display()))?;

    Ok(())
}

const DEFAULT_BEARING_STATUSES: &[&str] = &["exploring", "evaluating", "ready"];

/// Run the list command - shows bearings with status and EV score.
pub fn run_list(status_filters: &[String]) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let (config, _source) = load_config();
    let weights = config.current_weights();
    let status_filter = crate::cli::commands::management::status_filter::resolve_status_filter(
        status_filters,
        DEFAULT_BEARING_STATUSES,
        BEARING_STATUS_VALUES,
    )?;

    let rows = build_bearing_list_rows(&board_dir, &board, &status_filter, &weights);

    if rows.is_empty() {
        let hidden_terminal_statuses = ["laid", "parked", "declined"]
            .into_iter()
            .filter(|status| {
                !status_filter.contains(status)
                    && board
                        .bearings
                        .values()
                        .any(|bearing| bearing.frontmatter.status.to_string() == *status)
            })
            .collect::<Vec<_>>();
        let tip = crate::cli::commands::management::status_filter::build_append_status_suggestion(
            &hidden_terminal_statuses,
            "terminal bearings",
        );
        let state = crate::cli::commands::management::status_filter::build_empty_list_state(
            "bearings",
            !board.bearings.is_empty(),
            &status_filter,
            tip,
        );
        crate::cli::commands::management::status_filter::print_empty_list_state(&state);
        print_human(informational_for_list().as_ref());
        return Ok(());
    }

    let mut table = Table::new(&[
        "ID",
        "TITLE",
        "STATUS",
        "EVIDENCE",
        "ASSESS",
        "READINESS",
        "EV",
    ]);
    for row in rows {
        table.row(&[
            &row.id,
            &row.title,
            &row.status,
            &row.evidence,
            &row.assessment,
            &row.readiness,
            &row.ev,
        ]);
    }
    table.print();
    print_human(informational_for_list().as_ref());

    Ok(())
}

fn build_bearing_list_rows(
    board_dir: &Path,
    board: &Board,
    status_filter: &crate::cli::commands::management::status_filter::StatusFilter,
    weights: &keel::infrastructure::config::ModeWeights,
) -> Vec<BearingListRow> {
    let mut rows: Vec<(&Bearing, Option<f64>, String)> = board
        .bearings
        .values()
        .filter(|b| status_filter.contains(&b.frontmatter.status.to_string()))
        .map(|bearing| {
            let readiness = evaluate_bearing_readiness(board_dir, bearing, Some(weights));
            let score = readiness
                .score
                .as_ref()
                .map(|score| score.weighted_score)
                .or_else(|| get_bearing_score(board_dir, bearing, weights));
            (bearing, score, readiness.short_status())
        })
        .collect();

    rows.sort_by(|a, b| match (a.1, b.1) {
        (Some(score_a), Some(score_b)) => score_b.partial_cmp(&score_a).unwrap(),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.0.id().cmp(b.0.id()),
    });

    rows.into_iter()
        .map(|(bearing, score, readiness)| BearingListRow {
            id: bearing.id().to_string(),
            title: bearing.frontmatter.title.clone(),
            status: bearing.frontmatter.status.to_string(),
            evidence: if bearing.has_evidence { "✓" } else { "-" }.to_string(),
            assessment: if bearing.has_assessment { "✓" } else { "-" }.to_string(),
            readiness,
            ev: format_ev_score(score),
        })
        .collect()
}

/// Run the park command - shelve a bearing for later
pub fn run_park(pattern: &str) -> Result<()> {
    run_park_impl(pattern)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Park, pattern, err))
}

fn run_park_impl(pattern: &str) -> Result<()> {
    use keel::domain::transitions::bearing::{bearing_transitions, execute};

    let board_dir = find_board_dir()?;
    let result = execute(&board_dir, pattern, &bearing_transitions::PARK)?;

    println!("Parked: {} → {}", result.from, result.to);
    let guidance = guidance_for_action(BearingLifecycleAction::Park, &result.bearing_id);
    print_human(guidance.as_ref());

    Ok(())
}

/// Run the decline command - reject a bearing with reason
pub fn run_decline(pattern: &str, reason: &str) -> Result<()> {
    run_decline_impl(pattern, reason)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Decline, pattern, err))
}

fn run_decline_impl(pattern: &str, reason: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let bearing = board.require_bearing(pattern)?;

    let old_status = bearing.frontmatter.status;

    if old_status == BearingStatus::Declined {
        return Err(anyhow!("Bearing {} is already declined", bearing.id()));
    }

    update_bearing_status_with_decline(&board_dir, bearing, reason)?;

    println!("Declined: {} ({} → declined)", bearing.id(), old_status);
    println!("  Reason: {}", reason);
    let guidance = guidance_for_action(BearingLifecycleAction::Decline, bearing.id());
    print_human(guidance.as_ref());

    Ok(())
}

/// Update status to declined and add decline_reason to frontmatter
fn update_bearing_status_with_decline(
    board_dir: &std::path::Path,
    bearing: &Bearing,
    reason: &str,
) -> Result<()> {
    let readme_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("README.md");

    let content = fs::read_to_string(&readme_path)
        .with_context(|| format!("Failed to read README.md: {}", readme_path.display()))?;

    let updated = apply(
        &content,
        &[
            Mutation::set("status", "declined"),
            Mutation::set("decline_reason", format!("\"{}\"", reason)),
        ],
    );

    fs::write(&readme_path, updated)
        .with_context(|| format!("Failed to write README.md: {}", readme_path.display()))?;

    Ok(())
}

/// Run the lay command - graduate bearing to epic
pub fn run_lay(pattern: &str) -> Result<()> {
    run_lay_impl(pattern)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Lay, pattern, err))
}

fn run_lay_impl(pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_lay_at(&board_dir, pattern)
}

fn run_lay_at(board_dir: &Path, pattern: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let bearing = board.require_bearing(pattern)?;
    let (config, _source) = load_config();

    let old_status = bearing.frontmatter.status;

    // Check if bearing can be laid
    match old_status {
        BearingStatus::Declined => {
            return Err(anyhow!(
                "Cannot lay declined bearing: {}. Reason: {}",
                bearing.id(),
                bearing
                    .frontmatter
                    .decline_reason
                    .as_deref()
                    .unwrap_or("unknown")
            ));
        }
        BearingStatus::Parked => {
            return Err(anyhow!(
                "Cannot lay parked bearing: {}. Unpark it first.",
                bearing.id()
            ));
        }
        BearingStatus::Laid => {
            return Err(anyhow!("Bearing {} has already been laid", bearing.id()));
        }
        _ => {}
    }

    let brief_problems =
        keel::infrastructure::validation::bearings::check_bearing_content_sections_for_bearing(
            bearing,
        );
    if !brief_problems.is_empty() {
        return Err(anyhow!(
            "{}\nRecovery step:\n  keel bearing file {} BRIEF",
            brief_problems
                .into_iter()
                .map(|problem| problem.message)
                .collect::<Vec<_>>()
                .join("\n"),
            bearing.id()
        ));
    }

    let readiness = evaluate_bearing_readiness(board_dir, bearing, Some(&config.current_weights()));
    if !readiness.is_decision_ready() {
        let command = readiness
            .primary_recovery_command(bearing.id())
            .unwrap_or_else(|| format!("keel bearing show {}", bearing.id()));
        return Err(anyhow!(
            "{}\nRecovery step:\n  {}",
            readiness.problem_messages(bearing.id()).join("\n"),
            command
        ));
    }

    // Create the epic
    let epic_id = bearing.id();
    let epic_dir = board_dir.join("epics").join(epic_id);

    if epic_dir.exists() {
        return Err(anyhow!(
            "Epic already exists: {}. Choose a different bearing name.",
            epic_id
        ));
    }

    // Generate PRD content and validate goal references before any writes
    let prd_content = create_prd_from_bearing(board_dir, bearing)?;
    let brief_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("BRIEF.md");
    let brief_content = fs::read_to_string(&brief_path)
        .with_context(|| format!("Failed to read BRIEF.md: {}", brief_path.display()))?;
    let success_criteria =
        extract_section(&brief_content, "## Success Criteria").unwrap_or_default();
    let goal_refs = parse_goal_references(&success_criteria);
    if !goal_refs.is_empty() {
        let valid_goals = extract_prd_goal_ids(&prd_content);
        let invalid: Vec<_> = goal_refs
            .iter()
            .filter(|r| !valid_goals.contains(r))
            .collect();
        if !invalid.is_empty() {
            return Err(anyhow!(
                "Unknown goal reference(s) in BRIEF.md Success Criteria: {}. \
                 Valid goals in PRD: {}. \
                 Fix the references in bearings/{}/BRIEF.md before laying.",
                invalid
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                valid_goals.join(", "),
                bearing.id()
            ));
        }
    }

    fs::create_dir_all(&epic_dir)
        .with_context(|| format!("Failed to create epic directory: {}", epic_dir.display()))?;

    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    // Create README.md with bearing reference
    let readme_content = template_rendering::render(
        templates::epic::README,
        &[
            ("id", epic_id),
            ("title", &bearing.frontmatter.title),
            ("created_at", &now),
        ],
    );
    // Insert bearing reference after status line
    let readme_with_bearing = readme_content.replace(
        "status: planned",
        &format!("status: planned\nbearing: {}", epic_id),
    );
    let readme_path = epic_dir.join("README.md");
    fs::write(&readme_path, readme_with_bearing)
        .with_context(|| format!("Failed to write epic README: {}", readme_path.display()))?;

    // Write PRD.md (content already generated and validated above)
    let prd_path = epic_dir.join("PRD.md");
    fs::write(&prd_path, &prd_content)
        .with_context(|| format!("Failed to write PRD: {}", prd_path.display()))?;

    // Update bearing status to laid with validated goal references
    update_bearing_status(board_dir, bearing, BearingStatus::Laid, &goal_refs)?;

    println!("Laid: {} → epics/{}/", bearing.id(), epic_id);
    println!("  Epic created with PRD.md seeded from bearing documents");
    println!("  Bearing status: {} → laid", old_status);
    println!("  Lineage: epic={}", epic_id);
    if !goal_refs.is_empty() {
        println!("  Goals: {}", goal_refs.join(", "));
    }
    let guidance = guidance_for_action(BearingLifecycleAction::Lay, bearing.id());
    print_human(guidance.as_ref());

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(())
}

/// Create PRD content from bearing documents
fn create_prd_from_bearing(board_dir: &Path, bearing: &Bearing) -> Result<String> {
    let bearing_dir = board_dir.join("bearings").join(bearing.id());

    // Read BRIEF.md content
    let brief_path = bearing_dir.join("BRIEF.md");
    let brief_content = fs::read_to_string(&brief_path)
        .with_context(|| format!("Failed to read BRIEF.md: {}", brief_path.display()))?;

    // Extract sections from BRIEF.md
    let hypothesis = extract_section(&brief_content, "## Hypothesis").unwrap_or_default();
    let problem_space = extract_section(&brief_content, "## Problem Space").unwrap_or_default();
    let success_criteria =
        extract_section(&brief_content, "## Success Criteria").unwrap_or_default();

    // Read ASSESSMENT.md if it exists
    let assessment_path = bearing_dir.join("ASSESSMENT.md");
    let assessment_content = if assessment_path.exists() {
        Some(
            fs::read_to_string(&assessment_path)
                .with_context(|| "Failed to read ASSESSMENT.md".to_string())?,
        )
    } else {
        None
    };

    // Extract assessment analysis
    let mut analysis = String::new();
    if let Some(content) = assessment_content.as_ref() {
        for heading in [
            "## Findings",
            "## Opportunity Cost",
            "## Dependencies",
            "## Alternatives Considered",
        ] {
            if let Some(section) = extract_section(content, heading) {
                analysis.push_str(heading);
                analysis.push_str("\n\n");
                analysis.push_str(&section);
                analysis.push('\n');
            }
        }
    }

    // Build PRD content
    let mut prd = format!("# {} - Product Requirements\n\n", bearing.frontmatter.title);

    // Add hypothesis as value proposition
    if !hypothesis.is_empty() {
        prd.push_str(&format!("> {}\n\n", hypothesis.trim()));
    } else {
        prd.push_str("> Research-informed outcome for this epic.\n\n");
    }

    // Problem Statement from bearing
    prd.push_str("## Problem Statement\n\n");
    if !problem_space.is_empty() {
        prd.push_str(&format!("{}\n\n", problem_space.trim()));
    } else {
        prd.push_str("This epic addresses the validated problem surfaced by bearing research.\n\n");
    }

    prd.push_str("## Goals & Objectives\n\n");
    prd.push_str("| ID | Goal | Success Metric | Target |\n");
    prd.push_str("|----|------|----------------|--------|\n");
    prd.push_str("| GOAL-01 | Validate bearing recommendation in delivery flow | Adoption signal | Initial rollout complete |\n\n");

    prd.push_str("## Users\n\n");
    prd.push_str("| Persona | Description | Primary Need |\n");
    prd.push_str("|---------|-------------|--------------|\n");
    prd.push_str("| Product/Delivery Owner | Coordinates planning and execution | Reliable strategic direction |\n\n");

    prd.push_str("## Scope\n\n");
    prd.push_str("### In Scope\n\n");
    prd.push_str("- Deliver the bearing-backed capability slice for this epic.\n\n");
    prd.push_str("### Out of Scope\n\n");
    prd.push_str("- Unrelated platform-wide refactors outside bearing findings.\n\n");

    // Requirements
    prd.push_str("## Requirements\n\n");
    prd.push_str("### Functional Requirements\n\n");
    prd.push_str("<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->\n");
    prd.push_str("| ID | Requirement | Goals | Priority | Rationale |\n");
    prd.push_str("|----|-------------|-------|----------|-----------|\n");
    prd.push_str("| FR-01 | Implement the core user workflow identified in bearing research. | GOAL-01 | must | Converts research recommendation into executable product capability. |\n");
    prd.push_str("<!-- END FUNCTIONAL_REQUIREMENTS -->\n\n");

    prd.push_str("### Non-Functional Requirements\n\n");
    prd.push_str("<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->\n");
    prd.push_str("| ID | Requirement | Goals | Priority | Rationale |\n");
    prd.push_str("|----|-------------|-------|----------|-----------|\n");
    prd.push_str("| NFR-01 | Ensure deterministic behavior and operational visibility for the delivered workflow. | GOAL-01 | must | Keeps delivery safe and auditable during rollout. |\n");
    prd.push_str("<!-- END NON_FUNCTIONAL_REQUIREMENTS -->\n\n");

    prd.push_str("## Verification Strategy\n\n");
    prd.push_str("- Prove functional behavior through story-level verification evidence mapped to voyage requirements.\n");
    prd.push_str(
        "- Validate non-functional posture with operational checks and documented artifacts.\n\n",
    );

    prd.push_str("## Assumptions\n\n");
    prd.push_str("| Assumption | Impact if Wrong | Validation |\n");
    prd.push_str("|------------|-----------------|------------|\n");
    prd.push_str("| Bearing findings reflect current user needs | Scope may need re-planning | Re-check feedback during first voyage |\n\n");

    prd.push_str("## Open Questions & Risks\n\n");
    prd.push_str("| Question/Risk | Owner | Status |\n");
    prd.push_str("|---------------|-------|--------|\n");
    prd.push_str(
        "| Which rollout constraints should gate broader adoption? | Product | Open |\n\n",
    );

    // Success Criteria from bearing
    prd.push_str("## Success Criteria\n\n");
    prd.push_str("<!-- BEGIN SUCCESS_CRITERIA -->\n");
    if !success_criteria.is_empty() {
        let mut has_checkbox = false;
        for line in success_criteria.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("- [") {
                has_checkbox = true;
                prd.push_str(trimmed);
                prd.push('\n');
            }
        }
        if !has_checkbox {
            prd.push_str("- [ ] Bearing-backed workflow can be executed end-to-end in production conditions.\n");
        }
    } else {
        prd.push_str(
            "- [ ] Bearing-backed workflow can be executed end-to-end in production conditions.\n",
        );
    }
    prd.push_str("<!-- END SUCCESS_CRITERIA -->\n\n");

    // Analysis from assessment
    if !analysis.is_empty() {
        prd.push_str("## Research Analysis\n\n");
        prd.push_str("*From bearing assessment:*\n\n");
        prd.push_str(&format!("{}\n", analysis.trim()));
    }

    // Reference back to bearing
    prd.push_str("\n---\n\n");
    prd.push_str(&format!(
        "*This PRD was seeded from bearing `{}`. See `bearings/{}/` for original research.*\n",
        bearing.id(),
        bearing.id()
    ));

    Ok(prd)
}

use keel::infrastructure::validation::goals::{extract_prd_goal_ids, parse_goal_references};

/// Get the EV score for a bearing, if assessment exists and is complete
fn get_bearing_score(
    board_dir: &Path,
    bearing: &Bearing,
    weights: &keel::infrastructure::config::ModeWeights,
) -> Option<f64> {
    if !bearing.has_assessment {
        return None;
    }

    let assessment_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("ASSESSMENT.md");
    let evidence_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("EVIDENCE.md");

    load_bearing_score(&assessment_path, &evidence_path, weights)
        .ok()
        .map(|s| s.weighted_score)
}

/// Classify a bearing's exploration fog state.
fn classify_fog(board_dir: &Path, bearing: &Bearing) -> FogType {
    if bearing.has_evidence && bearing.has_assessment {
        return FogType::Clear;
    }

    let brief_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("BRIEF.md");

    let brief_content = fs::read_to_string(&brief_path).unwrap_or_default();
    let open_questions = extract_section(&brief_content, "## Open Questions").unwrap_or_default();
    let open_question_count = open_questions
        .lines()
        .filter(|line| line.trim_start().starts_with("- "))
        .count();
    let success_criteria =
        extract_section(&brief_content, "## Success Criteria").unwrap_or_default();
    let unchecked_criteria = success_criteria
        .lines()
        .filter(|line| line.trim_start().starts_with("- [ ]"))
        .count();

    if bearing.frontmatter.status == BearingStatus::Exploring {
        if !bearing.has_evidence && open_question_count > 0 {
            return FogType::Blocking;
        }

        if unchecked_criteria > 2 {
            return FogType::Blocking;
        }
    }

    match bearing.frontmatter.status {
        BearingStatus::Exploring | BearingStatus::Parked => FogType::Protecting,
        _ => FogType::Protecting,
    }
}

fn format_ev_score(score: Option<f64>) -> String {
    score
        .map(|value| format!("{value:.2}"))
        .unwrap_or_else(|| "-".to_string())
}

/// Format a factor value for display
fn format_factor(value: Option<u8>) -> String {
    value
        .map(|v| format!("{}/5", v))
        .unwrap_or_else(|| "-".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::presentation::flow::next_up::calculate_next_up;
    use keel::infrastructure::bearing_evidence::parse_evidence_records;
    use keel::infrastructure::config::ModeWeights;
    use keel::infrastructure::generate::board_readme::generate_board_readme;
    use keel::read_model::diagnostics::checks::bearings::check_bearing_assessment_recommendation;
    use keel::test_helpers::{TestBearing, TestBoardBuilder};
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    fn create_test_bearing(temp: &TempDir) -> PathBuf {
        create_test_bearing_with_status(temp, "exploring")
    }

    fn create_test_bearing_with_status(temp: &TempDir, status: &str) -> PathBuf {
        let root = temp.path();
        let bearing_dir = root.join("bearings/test-research");
        fs::create_dir_all(&bearing_dir).unwrap();

        fs::write(
            bearing_dir.join("README.md"),
            format!(
                r#"---
id: test-research
title: Test Research
status: {}
---
"#,
                status
            ),
        )
        .unwrap();

        fs::write(
            bearing_dir.join("BRIEF.md"),
            r#"# Test Research — Brief

## Hypothesis

The research should surface enough signal to justify a delivery epic.

## Problem Space

The team needs bearing framing that is explicit before downstream planning starts.

## Success Criteria

- [ ] The brief captures the bet clearly enough to guide research.

## Open Questions

- What evidence would overturn the current direction?
"#,
        )
        .unwrap();

        root.to_path_buf()
    }

    fn create_fog_test_bearing(
        temp: &TempDir,
        status: &str,
        has_evidence: bool,
        has_assessment: bool,
        open_questions: usize,
        unchecked_criteria: usize,
    ) -> PathBuf {
        let root = create_test_bearing_with_status(temp, status);
        let bearing_dir = root.join("bearings/test-research");

        let mut open_questions_section = String::new();
        for i in 0..open_questions {
            open_questions_section.push_str(&format!("- Question {}\n", i + 1));
        }

        let mut criteria_section = String::new();
        for i in 0..unchecked_criteria {
            criteria_section.push_str(&format!("- [ ] criterion {}\n", i + 1));
        }

        fs::write(
            bearing_dir.join("README.md"),
            format!(
                r#"---
id: test-research
title: Test Research
status: {}
---
"#,
                status
            ),
        )
        .unwrap();

        fs::write(
            bearing_dir.join("BRIEF.md"),
            format!(
                r#"# Test Research — Brief

## Hypothesis

The research direction is promising enough to merit deeper investigation.

## Problem Space

The operator needs a concrete problem statement before the bearing advances.

## Success Criteria
{}

## Open Questions
{}
"#,
                criteria_section, open_questions_section
            ),
        )
        .unwrap();

        if has_evidence {
            fs::write(bearing_dir.join("EVIDENCE.md"), "# Evidence").unwrap();
        }
        if has_assessment {
            fs::write(bearing_dir.join("ASSESSMENT.md"), "# Assessment").unwrap();
        }

        root
    }

    // Note: research/assess/park transition tests are in transitions/bearing_engine.rs
    // Command-level tests below verify other command workflows

    fn capture_request(
        class: EvidenceSourceClass,
        provider: &str,
        location: &str,
        notes: &str,
    ) -> ResearchCaptureRequest {
        ResearchCaptureRequest {
            class,
            provider: Some(provider.to_string()),
            location: location.to_string(),
            observed_or_published_at: chrono::NaiveDate::from_ymd_opt(2026, 3, 8).unwrap(),
            retrieved_at: chrono::NaiveDate::from_ymd_opt(2026, 3, 9).unwrap(),
            authority: EvidenceStrength::High,
            freshness: EvidenceStrength::Medium,
            notes: notes.to_string(),
        }
    }

    fn strong_evidence_fixture() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | academic | manual:prior-art-review | https://example.com/paper | 2026-02-01 | 2026-03-07 | high | high | Prior art supports the direction. |
| SRC-02 | web | manual:official-doc | https://example.com/docs | 2026-03-01 | 2026-03-08 | high | high | Official docs confirm feasibility. |
"#
    }

    fn weak_evidence_fixture() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | social | manual:community-signal | https://example.com/thread | 2020-03-01 | 2026-03-08 | low | low | One anecdotal signal supports the direction. |
| SRC-02 | social | manual:community-signal | https://example.com/thread-2 | 2020-04-01 | 2026-03-08 | low | low | A second anecdotal signal supports the direction. |
"#
    }

    fn narrow_evidence_fixture() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | web | manual:official-doc | https://example.com/docs | 2026-03-01 | 2026-03-08 | high | high | One strong source supports the direction. |
"#
    }

    fn cited_assessment_fixture() -> &'static str {
        r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 4 |
| Effort | 2 |
| Risk | 2 |


## Findings
- Delivery teams need source-backed recommendations before converting research into roadmap work [SRC-01][SRC-02]

## Opportunity Cost
Deferring this leaves roadmap work underspecified [SRC-02]

## Dependencies
- Evidence capture must produce canonical source records first [SRC-02]

## Alternatives Considered
- Keep factor-only scoring and trust operator judgment alone [SRC-01]

## Recommendation

[x] Proceed → convert to epic [SRC-01][SRC-02]
[ ] Park → revisit later [SRC-02]
[ ] Decline → document learnings [SRC-01]
"#
    }

    fn citation_broken_assessment_fixture() -> &'static str {
        r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 4 |
| Effort | 2 |
| Risk | 2 |


## Findings
- Delivery teams need source-backed recommendations before converting research into roadmap work [SRC-99]

## Opportunity Cost
Deferring this leaves roadmap work underspecified [SRC-99]

## Dependencies
- Evidence capture must produce canonical source records first [SRC-99]

## Alternatives Considered
- Keep factor-only scoring and trust operator judgment alone [SRC-99]

## Recommendation

[x] Proceed → convert to epic [SRC-99]
[ ] Park → revisit later [SRC-99]
[ ] Decline → document learnings [SRC-99]
"#
    }

    fn single_source_assessment_fixture() -> &'static str {
        r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 4 |
| Effort | 2 |
| Risk | 2 |


## Findings
- Delivery teams need source-backed recommendations before converting research into roadmap work [SRC-01]

## Opportunity Cost
Deferring this leaves roadmap work underspecified [SRC-01]

## Dependencies
- Evidence capture must produce canonical source records first [SRC-01]

## Alternatives Considered
- Keep factor-only scoring and trust operator judgment alone [SRC-01]

## Recommendation

[x] Proceed → convert to epic [SRC-01]
[ ] Park → revisit later [SRC-01]
[ ] Decline → document learnings [SRC-01]
"#
    }

    fn seed_readiness_docs(board_dir: &Path, id: &str, evidence: &str, assessment: &str) {
        let bearing_dir = board_dir.join("bearings").join(id);
        fs::write(bearing_dir.join("EVIDENCE.md"), evidence).unwrap();
        fs::write(bearing_dir.join("ASSESSMENT.md"), assessment).unwrap();
    }

    #[test]
    fn research_workflow_supports_all_signal_classes() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing(&temp);

        let first = run_research_capture(
            &board_dir,
            "test-research",
            capture_request(
                EvidenceSourceClass::Web,
                "manual:web-search",
                "https://example.com/web",
                "Web evidence",
            ),
        )
        .unwrap();
        assert!(matches!(
            first.transition,
            Some(ResearchTransitionSummary {
                from: BearingStatus::Exploring,
                to: BearingStatus::Evaluating,
                ..
            })
        ));

        run_research_capture(
            &board_dir,
            "test-research",
            capture_request(
                EvidenceSourceClass::Academic,
                "manual:arxiv",
                "https://arxiv.org/abs/1234.5678",
                "Academic evidence",
            ),
        )
        .unwrap();
        run_research_capture(
            &board_dir,
            "test-research",
            capture_request(
                EvidenceSourceClass::Social,
                "manual:social-trends",
                "https://news.ycombinator.com/item?id=42",
                "Social evidence",
            ),
        )
        .unwrap();
        run_research_capture(
            &board_dir,
            "test-research",
            capture_request(
                EvidenceSourceClass::Manual,
                "manual:internal-note",
                "docs/internal/research.md",
                "Manual evidence",
            ),
        )
        .unwrap();

        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();
        assert_eq!(bearing.status(), BearingStatus::Evaluating);

        let evidence =
            fs::read_to_string(board_dir.join("bearings/test-research/EVIDENCE.md")).unwrap();
        let records = parse_evidence_records(&evidence).unwrap();
        assert_eq!(
            records
                .iter()
                .map(|record| &record.class)
                .collect::<Vec<_>>(),
            vec![
                &EvidenceSourceClass::Web,
                &EvidenceSourceClass::Academic,
                &EvidenceSourceClass::Social,
                &EvidenceSourceClass::Manual,
            ]
        );
    }

    #[test]
    fn research_provider_status_is_explicit() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing(&temp);
        let mut config = keel::infrastructure::config::Config::default();
        config.research.providers.insert(
            "manual".to_string(),
            keel::infrastructure::config::ResearchProviderOverride {
                disabled: true,
                weight: 0.5,
            },
        );

        let disabled = run_research_capture_with_config(
            &board_dir,
            "test-research",
            capture_request(
                EvidenceSourceClass::Web,
                "manual:web-search",
                "https://example.com/web",
                "Web evidence",
            ),
            &config,
        )
        .unwrap_err()
        .to_string();
        assert!(disabled.contains("disabled"));
        assert!(disabled.contains("[research.providers.manual]"));

        let unavailable = run_research_capture_with_config(
            &board_dir,
            "test-research",
            capture_request(
                EvidenceSourceClass::Academic,
                "academic",
                "https://arxiv.org/abs/1234.5678",
                "Academic evidence",
            ),
            &keel::infrastructure::config::Config::default(),
        )
        .unwrap_err()
        .to_string();
        assert!(unavailable.contains("unavailable"));
        assert!(unavailable.contains("academic"));

        let unsupported = run_research_capture_with_config(
            &board_dir,
            "test-research",
            capture_request(
                EvidenceSourceClass::Social,
                "trend-scout",
                "https://news.ycombinator.com/item?id=42",
                "Social evidence",
            ),
            &keel::infrastructure::config::Config::default(),
        )
        .unwrap_err()
        .to_string();
        assert!(unsupported.contains("unsupported"));
        assert!(unsupported.contains("manual, web, academic, social"));
    }

    #[test]
    fn research_provider_failures_do_not_fabricate_evidence() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing(&temp);

        let err = run_research_capture_with_config(
            &board_dir,
            "test-research",
            capture_request(
                EvidenceSourceClass::Web,
                "web",
                "https://example.com/web",
                "Web evidence",
            ),
            &keel::infrastructure::config::Config::default(),
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("unavailable"));

        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();
        assert_eq!(bearing.status(), BearingStatus::Exploring);
        assert!(
            !board_dir
                .join("bearings/test-research/EVIDENCE.md")
                .exists()
        );
    }

    #[test]
    fn park_updates_status() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing(&temp);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        update_bearing_status(&board_dir, bearing, BearingStatus::Parked, &[]).unwrap();

        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        assert!(readme.contains("status: parked"));
    }

    #[test]
    fn decline_updates_status_and_adds_reason() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing(&temp);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        update_bearing_status_with_decline(&board_dir, bearing, "Not viable").unwrap();

        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        assert!(readme.contains("status: declined"));
        assert!(readme.contains("decline_reason: \"Not viable\""));
    }

    #[test]
    fn extract_section_finds_content() {
        let content = r#"# Title

## Hypothesis

This is the hypothesis content.
It spans multiple lines.

## Problem Space

Different section content.
"#;
        let hypothesis = extract_section(content, "## Hypothesis").unwrap();
        assert!(hypothesis.contains("hypothesis content"));
        assert!(hypothesis.contains("multiple lines"));
        assert!(!hypothesis.contains("Different section"));
    }

    #[test]
    fn create_prd_from_bearing_includes_reference() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing(&temp);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        let prd = create_prd_from_bearing(&board_dir, bearing).unwrap();

        assert!(prd.contains("Test Research - Product Requirements"));
        assert!(prd.contains("seeded from bearing `test-research`"));
        assert!(prd.contains("bearings/test-research/"));
    }

    #[test]
    fn classify_fog_blocking() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_fog_test_bearing(&temp, "exploring", false, false, 2, 0);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        assert_eq!(classify_fog(&board_dir, bearing), FogType::Blocking);
    }

    #[test]
    fn classify_fog_protecting() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_fog_test_bearing(&temp, "exploring", true, false, 1, 0);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        assert_eq!(classify_fog(&board_dir, bearing), FogType::Protecting);
    }

    #[test]
    fn classify_fog_clear() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_fog_test_bearing(&temp, "exploring", true, true, 8, 5);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        assert_eq!(classify_fog(&board_dir, bearing), FogType::Clear);
    }

    #[test]
    fn bearing_readiness_requires_evidence_quality() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "ready");
        seed_readiness_docs(
            &board_dir,
            "test-research",
            narrow_evidence_fixture(),
            single_source_assessment_fixture(),
        );

        let board = load_board(&board_dir).unwrap();
        let doctor_problems = check_bearing_assessment_recommendation(&board, &board_dir);
        assert!(
            doctor_problems
                .iter()
                .any(|problem| problem.message.contains("not decision-ready"))
        );
        assert!(
            doctor_problems
                .iter()
                .any(|problem| problem.message.contains("too narrow"))
        );

        let error = run_lay_at(&board_dir, "test-research")
            .unwrap_err()
            .to_string();
        assert!(error.contains("not decision-ready"));
        assert!(error.contains("keel bearing file test-research EVIDENCE"));
    }

    #[test]
    fn bearing_lay_requires_authored_brief_framing() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "ready");
        seed_readiness_docs(
            &board_dir,
            "test-research",
            strong_evidence_fixture(),
            cited_assessment_fixture(),
        );
        fs::write(
            board_dir.join("bearings/test-research/BRIEF.md"),
            r#"# Test Research — Brief

## Hypothesis

What we believe and why it might matter.

## Problem Space

What problem or opportunity are we investigating?

## Success Criteria

- [ ] Criterion 1
- [ ] Criterion 2

## Open Questions

- Question we need to answer
"#,
        )
        .unwrap();

        let error = run_lay_at(&board_dir, "test-research")
            .unwrap_err()
            .to_string();
        assert!(error.contains("BRIEF.md Hypothesis scaffold"));
        assert!(error.contains("keel bearing file test-research BRIEF"));
    }

    #[test]
    fn bearing_lay_persists_epic_lineage_field() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "ready");
        seed_readiness_docs(
            &board_dir,
            "test-research",
            strong_evidence_fixture(),
            cited_assessment_fixture(),
        );

        run_lay_at(&board_dir, "test-research").unwrap();

        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        assert!(
            readme.contains("status: laid"),
            "bearing status must be laid"
        );
        assert!(
            readme.contains("epic: test-research"),
            "epic lineage field must be the bearing ID (which becomes the epic ID)"
        );
        assert!(readme.contains("laid_at:"), "laid_at must be set");
    }

    #[test]
    fn bearing_lay_epic_field_preserves_existing_frontmatter() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "ready");
        seed_readiness_docs(
            &board_dir,
            "test-research",
            strong_evidence_fixture(),
            cited_assessment_fixture(),
        );

        run_lay_at(&board_dir, "test-research").unwrap();

        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        // Original fields preserved
        assert!(readme.contains("id: test-research"));
        assert!(readme.contains("title: Test Research"));
        // New lineage field added
        assert!(readme.contains("epic: test-research"));
    }

    #[test]
    fn bearing_projections_surface_evidence_quality() {
        let temp = TestBoardBuilder::new()
            .bearing(
                TestBearing::new("1w5H2Bq9L")
                    .status("ready")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .bearing(
                TestBearing::new("1w5H2Bq9M")
                    .status("ready")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .build();

        seed_readiness_docs(
            temp.path(),
            "1w5H2Bq9L",
            weak_evidence_fixture(),
            cited_assessment_fixture(),
        );
        seed_readiness_docs(
            temp.path(),
            "1w5H2Bq9M",
            strong_evidence_fixture(),
            cited_assessment_fixture(),
        );

        let board = load_board(temp.path()).unwrap();
        let status_filter = crate::cli::commands::management::status_filter::resolve_status_filter(
            &["ready".to_string()],
            DEFAULT_BEARING_STATUSES,
            BEARING_STATUS_VALUES,
        )
        .unwrap();
        let rows =
            build_bearing_list_rows(temp.path(), &board, &status_filter, &ModeWeights::growth());
        assert!(rows.iter().any(|row| {
            row.id == "1w5H2Bq9L" && row.readiness == "raise authority" && row.ev != "-"
        }));
        assert!(rows.iter().any(|row| {
            row.id == "1w5H2Bq9M" && row.readiness == "decision-ready" && row.ev != "-"
        }));

        let next_up = calculate_next_up(&board);
        assert!(
            next_up
                .human_items
                .iter()
                .any(|item| item.id == "1w5H2Bq9L" && item.category == "research")
        );

        let readme = generate_board_readme(&board);
        assert!(
            readme.contains("| Bearing | Status | Evidence | Assessment | Readiness | EV | Laid |")
        );
        assert!(readme.contains("raise authority"));
        assert!(readme.contains("decision-ready"));
    }

    #[test]
    fn bearing_readiness_guidance_targets_missing_evidence() {
        let evidence_temp = TempDir::new().unwrap();
        let evidence_board_dir = create_test_bearing_with_status(&evidence_temp, "ready");
        seed_readiness_docs(
            &evidence_board_dir,
            "test-research",
            narrow_evidence_fixture(),
            single_source_assessment_fixture(),
        );
        let evidence_error = run_lay_at(&evidence_board_dir, "test-research")
            .unwrap_err()
            .to_string();
        assert!(evidence_error.contains("keel bearing file test-research EVIDENCE"));

        let citation_temp = TempDir::new().unwrap();
        let citation_board_dir = create_test_bearing_with_status(&citation_temp, "ready");
        seed_readiness_docs(
            &citation_board_dir,
            "test-research",
            strong_evidence_fixture(),
            citation_broken_assessment_fixture(),
        );
        let citation_error = run_lay_at(&citation_board_dir, "test-research")
            .unwrap_err()
            .to_string();
        assert!(citation_error.contains("keel bearing file test-research ASSESSMENT"));
    }

    #[test]
    fn parse_goal_references_extracts_goal_ids() {
        let criteria = r#"
- [ ] GOAL-01: First criterion
- [ ] Second criterion (no goal ref)
- [ ] GOAL-02: Third criterion references GOAL-01 again
"#;
        let refs = parse_goal_references(criteria);
        assert_eq!(refs, vec!["GOAL-01", "GOAL-02"]);
    }

    #[test]
    fn parse_goal_references_returns_empty_for_no_goals() {
        let criteria = "- [ ] Plain criterion\n- [ ] Another one\n";
        assert!(parse_goal_references(criteria).is_empty());
    }

    #[test]
    fn extract_prd_goal_ids_parses_goals_table() {
        let prd = r#"## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | First goal | Metric | Target |
| GOAL-02 | Second goal | Metric | Target |
"#;
        let ids = extract_prd_goal_ids(prd);
        assert_eq!(ids, vec!["GOAL-01", "GOAL-02"]);
    }

    #[test]
    fn bearing_lay_rejects_unknown_goal_references() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "ready");
        seed_readiness_docs(
            &board_dir,
            "test-research",
            strong_evidence_fixture(),
            cited_assessment_fixture(),
        );
        // Write BRIEF.md with an invalid goal reference
        fs::write(
            board_dir.join("bearings/test-research/BRIEF.md"),
            r#"# Test Research — Brief

## Hypothesis

Explicit goal references in research briefs should flow into laid-bearing lineage.

## Problem Space

Goal-link validation should fail fast without allowing the bearing to lay partial lineage state.

## Success Criteria

- [ ] GOAL-99: This goal does not exist in the PRD

## Open Questions

- How should lay surface invalid goal-link recovery guidance?
"#,
        )
        .unwrap();

        let error = run_lay_at(&board_dir, "test-research")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("GOAL-99"),
            "error must name the offending goal reference"
        );
        assert!(
            error.contains("Unknown goal reference"),
            "error must describe the validation failure"
        );
        // Verify no writes occurred — epic directory must not exist
        assert!(
            !board_dir.join("epics/test-research").exists(),
            "no writes should occur before validation"
        );
    }

    #[test]
    fn bearing_lay_persists_valid_goal_references() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "ready");
        seed_readiness_docs(
            &board_dir,
            "test-research",
            strong_evidence_fixture(),
            cited_assessment_fixture(),
        );
        // Write BRIEF.md with a valid goal reference
        fs::write(
            board_dir.join("bearings/test-research/BRIEF.md"),
            r#"# Test Research — Brief

## Hypothesis

Explicit goal references in research briefs should survive the lay transition.

## Problem Space

Validated brief goals need to persist as machine-readable bearing lineage.

## Success Criteria

- [ ] GOAL-01: Valid criterion matching PRD goal

## Open Questions

- Should multiple valid goals preserve deterministic ordering?
"#,
        )
        .unwrap();

        run_lay_at(&board_dir, "test-research").unwrap();

        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        assert!(
            readme.contains("goals: [GOAL-01]"),
            "valid goal references must be persisted in bearing frontmatter"
        );
    }
}
