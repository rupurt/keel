//! Bootstrap a new Keel project scaffold.

use anyhow::{Context, Result, anyhow};
use keel::infrastructure::{
    config::Config,
    generate::{BoardArtifactSyncOptions, sync_board_artifacts},
    loader::load_board,
    template_rendering, templates,
};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const BOARD_SUBDIRS: [&str; 12] = [
    "stories",
    "epics",
    "bearings",
    "adrs",
    "missions",
    "routines",
    "watches",
    "knowledge",
    "inbox",
    "outbox",
    "cache",
    "play",
];

const GITIGNORE_ENTRIES: [&str; 3] = [".keel/inbox/", ".keel/outbox/", ".keel/cache/"];

#[derive(Clone, Copy)]
struct FoundationalDocSpec {
    path: &'static str,
    description: &'static str,
    template: &'static str,
}

const FOUNDATIONAL_DOCS: [FoundationalDocSpec; 8] = [
    FoundationalDocSpec {
        path: "CONSTITUTION.md",
        description: "collaboration philosophy and decision hierarchy",
        template: templates::project::CONSTITUTION,
    },
    FoundationalDocSpec {
        path: "POLICY.md",
        description: "operational invariants and safety rails",
        template: templates::project::POLICY,
    },
    FoundationalDocSpec {
        path: "ARCHITECTURE.md",
        description: "system map, boundaries, and technical seams",
        template: templates::project::ARCHITECTURE,
    },
    FoundationalDocSpec {
        path: "USER_GUIDE.md",
        description: "operator-visible product story and workflow guidance",
        template: templates::project::USER_GUIDE,
    },
    FoundationalDocSpec {
        path: "AGENTS.md",
        description: "repo-wide AI entry contract with upstream sync seam",
        template: templates::project::AGENTS,
    },
    FoundationalDocSpec {
        path: "GEMINI.md",
        description: "Gemini harness adapter that points back to the shared contract",
        template: templates::project::GEMINI,
    },
    FoundationalDocSpec {
        path: "CLAUDE.md",
        description: "Claude harness adapter that points back to the shared contract",
        template: templates::project::CLAUDE,
    },
    FoundationalDocSpec {
        path: "INSTRUCTIONS.md",
        description: "procedural turn loop and hygiene checklist with sync seam",
        template: templates::project::INSTRUCTIONS,
    },
];

#[derive(Debug, Default, PartialEq, Eq)]
struct ScaffoldReport {
    created_docs: Vec<&'static str>,
    replaced_docs: Vec<&'static str>,
    kept_docs: Vec<&'static str>,
    config_created: bool,
    gitignore_created: bool,
    gitignore_updated: bool,
    initialized_git: bool,
    used_existing_git: bool,
    hooks_installed: bool,
    board_synced: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScaffoldContext {
    project_name: String,
    board_dir: String,
}

/// Create a new Keel project scaffold in the target directory.
pub fn run(target: Option<&Path>) -> Result<()> {
    let explicit_target = target.filter(|path| *path != Path::new("."));
    let root = prepare_target_root(target.unwrap_or(Path::new(".")))?;
    let config = Config::default();
    let report = scaffold_project(&root, &config, explicit_target.is_some(), |path| {
        confirm_replace(path)
    })?;
    print_report(&root, &config, explicit_target, &report);
    Ok(())
}

fn prepare_target_root(target: &Path) -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    let root = if target.is_absolute() {
        target.to_path_buf()
    } else {
        cwd.join(target)
    };

    if root.exists() && !root.is_dir() {
        return Err(anyhow!(
            "Target path '{}' exists but is not a directory",
            root.display()
        ));
    }

    fs::create_dir_all(&root)
        .with_context(|| format!("Failed to create target directory {}", root.display()))?;

    Ok(root)
}

fn scaffold_project<F>(
    root: &Path,
    config: &Config,
    explicit_new_project: bool,
    mut confirm_replace: F,
) -> Result<ScaffoldReport>
where
    F: FnMut(&Path) -> Result<bool>,
{
    let mut report = ScaffoldReport::default();
    let board_path = ensure_board_scaffold(root, config)?;

    if write_config_if_missing(root, config)? {
        report.config_created = true;
    }

    let context = ScaffoldContext {
        project_name: humanize_project_name(project_name_from_root(root)),
        board_dir: config.board_dir().to_string(),
    };
    let doc_report = scaffold_foundational_docs(root, &context, &mut confirm_replace)?;
    report.created_docs = doc_report.created_docs;
    report.replaced_docs = doc_report.replaced_docs;
    report.kept_docs = doc_report.kept_docs;

    let gitignore_status = ensure_gitignore(root)?;
    report.gitignore_created = gitignore_status.created;
    report.gitignore_updated = gitignore_status.updated;

    let git_status = ensure_git_repository(root, explicit_new_project)?;
    report.initialized_git = git_status.initialized_here;
    report.used_existing_git = git_status.used_existing_repo;

    match super::hooks::run_in(Some(root)) {
        Ok(()) => report.hooks_installed = true,
        Err(err) => eprintln!("Warning: Could not install git hooks: {err}"),
    }

    let board = load_board(&board_path)
        .with_context(|| format!("Failed to load board scaffold at {}", board_path.display()))?;
    sync_board_artifacts(&board, &BoardArtifactSyncOptions::default())
        .context("Failed to generate initial board artifacts")?;
    report.board_synced = true;

    Ok(report)
}

fn ensure_board_scaffold(root: &Path, config: &Config) -> Result<PathBuf> {
    let board_path = root.join(config.board_dir());
    if board_path.exists() && !board_path.is_dir() {
        return Err(anyhow!(
            "Board path '{}' exists but is not a directory",
            board_path.display()
        ));
    }

    fs::create_dir_all(&board_path)
        .with_context(|| format!("Failed to create board directory {}", board_path.display()))?;

    for dir in BOARD_SUBDIRS {
        let dir_path = board_path.join(dir);
        fs::create_dir_all(&dir_path).with_context(|| {
            format!("Failed to create board subdirectory {}", dir_path.display())
        })?;
    }

    Ok(board_path)
}

fn write_config_if_missing(root: &Path, config: &Config) -> Result<bool> {
    let config_path = root.join("keel.toml");
    if config_path.exists() {
        return Ok(false);
    }

    let content = template_rendering::render(
        templates::project::KEEL_TOML,
        &[("board_dir", config.board_dir())],
    );
    fs::write(&config_path, content)
        .with_context(|| format!("Failed to write {}", config_path.display()))?;
    Ok(true)
}

fn scaffold_foundational_docs<F>(
    root: &Path,
    context: &ScaffoldContext,
    mut confirm_replace: F,
) -> Result<ScaffoldReport>
where
    F: FnMut(&Path) -> Result<bool>,
{
    let mut report = ScaffoldReport::default();

    for spec in FOUNDATIONAL_DOCS {
        let path = root.join(spec.path);
        if path.exists() {
            if confirm_replace(&path)? {
                write_foundational_doc(&path, spec, context)?;
                report.replaced_docs.push(spec.path);
            } else {
                report.kept_docs.push(spec.path);
            }
            continue;
        }

        write_foundational_doc(&path, spec, context)?;
        report.created_docs.push(spec.path);
    }

    Ok(report)
}

fn write_foundational_doc(
    path: &Path,
    spec: FoundationalDocSpec,
    context: &ScaffoldContext,
) -> Result<()> {
    let body = template_rendering::render(
        spec.template,
        &[
            ("project_name", &context.project_name),
            ("board_dir", &context.board_dir),
        ],
    );
    fs::write(path, body).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

#[derive(Debug, Default, PartialEq, Eq)]
struct GitignoreStatus {
    created: bool,
    updated: bool,
}

fn ensure_gitignore(root: &Path) -> Result<GitignoreStatus> {
    let gitignore_path = root.join(".gitignore");
    let mut status = GitignoreStatus::default();
    let existing = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)
            .with_context(|| format!("Failed to read {}", gitignore_path.display()))?
    } else {
        status.created = true;
        String::new()
    };

    let missing: Vec<_> = GITIGNORE_ENTRIES
        .iter()
        .copied()
        .filter(|entry| !existing.contains(entry))
        .collect();
    if missing.is_empty() && !status.created {
        return Ok(status);
    }

    let mut next = existing;
    if !next.is_empty() && !next.ends_with('\n') {
        next.push('\n');
    }
    if !missing.is_empty() {
        if !next.is_empty() {
            next.push('\n');
        }
        next.push_str("# Keel ignored artifacts\n");
        next.push_str(&missing.join("\n"));
        next.push('\n');
    }

    fs::write(&gitignore_path, next)
        .with_context(|| format!("Failed to write {}", gitignore_path.display()))?;
    status.updated = !missing.is_empty();
    Ok(status)
}

#[derive(Debug, Default, PartialEq, Eq)]
struct GitRepositoryStatus {
    initialized_here: bool,
    used_existing_repo: bool,
}

fn ensure_git_repository(root: &Path, explicit_new_project: bool) -> Result<GitRepositoryStatus> {
    if root.join(".git").exists() {
        return Ok(GitRepositoryStatus {
            initialized_here: false,
            used_existing_repo: true,
        });
    }

    if explicit_new_project {
        init_git_repo(root)?;
        return Ok(GitRepositoryStatus {
            initialized_here: true,
            used_existing_repo: false,
        });
    }

    if is_inside_git_repo(root) {
        return Ok(GitRepositoryStatus {
            initialized_here: false,
            used_existing_repo: true,
        });
    }

    init_git_repo(root)?;
    Ok(GitRepositoryStatus {
        initialized_here: true,
        used_existing_repo: false,
    })
}

fn init_git_repo(root: &Path) -> Result<()> {
    let status = std::process::Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(root)
        .status()
        .with_context(|| format!("Failed to initialize git repository in {}", root.display()))?;

    if !status.success() {
        return Err(anyhow!("git init failed in {}", root.display()));
    }

    Ok(())
}

fn is_inside_git_repo(root: &Path) -> bool {
    std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(root)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn confirm_replace(path: &Path) -> Result<bool> {
    print!(
        "Replace existing {} with the Keel downstream template? [y/N]: ",
        path.display()
    );
    io::stdout().flush().context("Failed to flush stdout")?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read replacement confirmation")?;

    Ok(matches!(
        input.trim().to_ascii_lowercase().as_str(),
        "y" | "yes"
    ))
}

fn project_name_from_root(root: &Path) -> &str {
    root.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("Project")
}

fn humanize_project_name(raw: &str) -> String {
    let words: Vec<_> = raw
        .split(['-', '_', ' '])
        .filter(|part| !part.is_empty())
        .map(capitalize_word)
        .collect();

    if words.is_empty() {
        "Project".to_string()
    } else {
        words.join(" ")
    }
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut output = String::new();
    output.extend(first.to_uppercase());
    output.push_str(chars.as_str());
    output
}

fn print_report(
    root: &Path,
    config: &Config,
    explicit_target: Option<&Path>,
    report: &ScaffoldReport,
) {
    println!("Created Keel project scaffold at {}", root.display());
    println!(
        "Board directory: {}",
        root.join(config.board_dir()).display()
    );

    if report.initialized_git {
        println!("Initialized a new git repository.");
    } else if report.used_existing_git {
        println!("Using the existing git repository for hook installation.");
    }

    if report.config_created {
        println!("Created keel.toml with downstream workflow defaults.");
    } else {
        println!("Found existing keel.toml. Kept the current project configuration.");
    }

    if report.gitignore_created {
        println!("Created .gitignore with Keel cache and inbox ignores.");
    } else if report.gitignore_updated {
        println!("Updated .gitignore with Keel cache and inbox ignores.");
    }

    if report.hooks_installed {
        println!("Installed git hooks for commit-boundary validation.");
    }

    if !report.created_docs.is_empty() {
        println!("\nCreated foundational downstream docs:");
        for doc in &report.created_docs {
            println!("  - {doc}");
        }
    }

    if !report.replaced_docs.is_empty() {
        println!("\nReplaced foundational docs from the Keel templates:");
        for doc in &report.replaced_docs {
            println!("  - {doc}");
        }
    }

    if !report.kept_docs.is_empty() {
        println!("\nKept existing downstream docs:");
        for doc in &report.kept_docs {
            println!("  - {doc}");
        }
    }

    println!("\nHydrate the project-specific foundation docs before you start delivery work:");
    for spec in FOUNDATIONAL_DOCS {
        println!("  - {} — {}", spec.path, spec.description);
    }
    println!("  - keel.toml — tune board_dir, workflow defaults, roles, and lanes for the repo");

    println!("\nAGENTS.md and INSTRUCTIONS.md are scaffolded as downstream copies.");
    println!("Preserve their `PROJECT-SPECIFIC` blocks when you sync newer Keel versions.");

    if explicit_target.is_some() {
        println!("\nNext:");
        println!("  cd {}", root.display());
    }
    println!("  keel turn");
    println!("  keel mission new \"Describe the first strategic outcome\"");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn new_scaffolds_board_config_and_foundations() {
        let temp = tempdir().unwrap();
        let root = temp.path().join("harbor");

        let report = scaffold_project(&root, &Config::default(), true, |_| Ok(false)).unwrap();

        assert!(root.join(".git").is_dir());
        assert!(root.join(".git/hooks/pre-commit").exists());
        assert!(root.join(".git/hooks/commit-msg").exists());
        assert!(root.join(".keel").is_dir());
        assert!(root.join(".keel/stories").is_dir());
        assert!(root.join(".keel/missions").is_dir());
        assert!(root.join(".keel/play").is_dir());
        assert!(root.join(".keel/README.md").is_file());
        assert!(root.join("keel.toml").is_file());
        assert!(root.join("AGENTS.md").is_file());
        assert!(root.join("INSTRUCTIONS.md").is_file());
        assert!(root.join("GEMINI.md").is_file());
        assert!(root.join("CLAUDE.md").is_file());
        assert!(report.initialized_git);
        assert!(report.config_created);
        assert!(report.hooks_installed);
        assert!(report.board_synced);

        let config = fs::read_to_string(root.join("keel.toml")).unwrap();
        assert!(config.contains("board_dir = \".keel\""));
        assert!(config.contains("[workflow]"));
        assert!(config.contains("working_hours_start = 7"));
        assert!(config.contains("[roles.manager]"));
    }

    #[test]
    fn new_keeps_existing_doc_when_prompt_declines() {
        let temp = tempdir().unwrap();
        let root = temp.path().join("harbor");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("AGENTS.md"), "# Custom\n").unwrap();

        let report = scaffold_project(&root, &Config::default(), true, |_| Ok(false)).unwrap();

        assert_eq!(
            fs::read_to_string(root.join("AGENTS.md")).unwrap(),
            "# Custom\n"
        );
        assert!(report.kept_docs.contains(&"AGENTS.md"));
        assert!(!report.replaced_docs.contains(&"AGENTS.md"));
    }

    #[test]
    fn new_replaces_existing_doc_when_prompt_accepts() {
        let temp = tempdir().unwrap();
        let root = temp.path().join("harbor");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("AGENTS.md"), "# Custom\n").unwrap();

        let report = scaffold_project(&root, &Config::default(), true, |path| {
            Ok(path.file_name().and_then(|name| name.to_str()) == Some("AGENTS.md"))
        })
        .unwrap();

        let agents = fs::read_to_string(root.join("AGENTS.md")).unwrap();
        assert!(agents.contains("Shared guidance for AI agents"));
        assert!(agents.contains("BEGIN PROJECT-SPECIFIC"));
        assert!(report.replaced_docs.contains(&"AGENTS.md"));
    }
}
