//! `keel upgrade` — install the latest release or a locally built git ref.

use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Context, Result, bail};
use tempfile::Builder;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const CARGO_DIST_REPOSITORY_SLUG: &str = "axodotdev/cargo-dist";

pub fn run(ref_spec: Option<&str>) -> Result<()> {
    let installer = InstallerPlatform::current();
    match ref_spec {
        Some(ref_spec) => install_from_ref(ref_spec, installer),
        None => install_latest_release(installer),
    }
}

fn install_latest_release(installer: InstallerPlatform) -> Result<()> {
    let cache_root = ensure_cache_root()?;
    let temp_root = cache_root.join("tmp");
    fs::create_dir_all(&temp_root)
        .with_context(|| format!("Failed to create {}", temp_root.display()))?;

    let download_dir = Builder::new()
        .prefix("upgrade-latest-")
        .tempdir_in(&temp_root)
        .context("Failed to create a temporary download directory")?;
    let script_path = download_dir.path().join(installer.script_name);
    let url = format!(
        "https://github.com/{}/releases/latest/download/{}",
        github_repo_slug(APP_REPOSITORY)?,
        installer.script_name
    );

    println!("Installing the latest released {APP_NAME} build...");
    download_remote_script(&url, &script_path)?;
    run_installer_script(&script_path, installer, None)?;
    println!("Installed {APP_NAME} from the latest published release.");

    Ok(())
}

fn install_from_ref(ref_spec: &str, installer: InstallerPlatform) -> Result<()> {
    let cache_root = ensure_cache_root()?;
    let repo_dir = ensure_cached_repo(&cache_root)?;
    refresh_cached_repo(&repo_dir)?;

    let resolved = resolve_git_ref(&repo_dir, ref_spec)?;
    let short = short_sha(&resolved);
    let checkout = CachedWorktree::checkout(&repo_dir, &resolved, &cache_root)?;

    ensure_dist_package_opt_in(checkout.path())?;

    let toolchain = discover_supported_toolchain(checkout.path())?;
    let host_target = detect_host_target(&toolchain, checkout.path())?;
    let dist_version = load_cargo_dist_version(checkout.path())?;
    let dist_bin = ensure_cargo_dist(&cache_root, &dist_version, installer)?;
    let build_root = cache_root
        .join("source-builds")
        .join(format!("{short}-{host_target}"));

    println!("Preparing cached source for {ref_spec} ({short})...");
    println!("Using Rust toolchain: {}", toolchain.label);
    println!("Building a local installer bundle for {host_target}...");

    let distrib_dir = build_local_installer_bundle(
        checkout.path(),
        &toolchain,
        &dist_bin,
        &host_target,
        &build_root,
    )?;

    drop(checkout);

    let script_path = distrib_dir.join(installer.script_name);
    if !script_path.exists() {
        bail!("Local dist build did not produce {}", script_path.display());
    }

    println!("Running the locally built installer...");
    run_installer_script(
        &script_path,
        installer,
        Some(&file_url_for_dir(&distrib_dir)?),
    )?;
    println!("Installed {APP_NAME} from {ref_spec} ({short}).");

    Ok(())
}

fn ensure_cache_root() -> Result<PathBuf> {
    let cache_root = dirs::cache_dir()
        .or_else(|| dirs::home_dir().map(|home| home.join(".cache")))
        .ok_or_else(|| anyhow::anyhow!("Could not resolve a cache directory for {APP_NAME}"))?
        .join(APP_NAME);
    fs::create_dir_all(&cache_root)
        .with_context(|| format!("Failed to create {}", cache_root.display()))?;
    Ok(cache_root)
}

fn ensure_cached_repo(cache_root: &Path) -> Result<PathBuf> {
    let repo_dir = cache_root.join("source");
    if repo_dir.exists() {
        return Ok(repo_dir);
    }

    let parent = repo_dir
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid cache repo path: {}", repo_dir.display()))?;
    fs::create_dir_all(parent).with_context(|| format!("Failed to create {}", parent.display()))?;

    let mut cmd = Command::new("git");
    cmd.arg("clone")
        .arg("--quiet")
        .arg("--filter=blob:none")
        .arg(APP_REPOSITORY)
        .arg(&repo_dir);
    run_command(&mut cmd, "clone the cached keel source repository")?;

    Ok(repo_dir)
}

fn refresh_cached_repo(repo_dir: &Path) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(repo_dir)
        .arg("fetch")
        .arg("--quiet")
        .arg("--tags")
        .arg("--prune")
        .arg("origin");
    run_command(&mut cmd, "refresh the cached keel source repository")
}

fn resolve_git_ref(repo_dir: &Path, ref_spec: &str) -> Result<String> {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(repo_dir)
        .arg("rev-parse")
        .arg("--verify")
        .arg(format!("{ref_spec}^{{commit}}"));
    read_stdout(&mut cmd, &format!("resolve git ref `{ref_spec}`"))
}

fn short_sha(sha: &str) -> &str {
    sha.get(..12).unwrap_or(sha)
}

fn load_cargo_dist_version(checkout_dir: &Path) -> Result<String> {
    let workspace_dist = checkout_dir.join("dist-workspace.toml");
    if workspace_dist.exists() {
        let contents = fs::read_to_string(&workspace_dist)
            .with_context(|| format!("Failed to read {}", workspace_dist.display()))?;
        if let Some(version) = parse_dist_workspace_version(&contents)? {
            return Ok(version);
        }
    }

    let cargo_toml = checkout_dir.join("Cargo.toml");
    let contents = fs::read_to_string(&cargo_toml)
        .with_context(|| format!("Failed to read {}", cargo_toml.display()))?;
    if let Some(version) = parse_root_cargo_dist_version(&contents)? {
        return Ok(version);
    }

    bail!(
        "Could not find a cargo-dist version in {} or {}",
        workspace_dist.display(),
        cargo_toml.display()
    )
}

fn parse_dist_workspace_version(contents: &str) -> Result<Option<String>> {
    let value: toml::Value =
        toml::from_str(contents).context("Failed to parse dist-workspace.toml")?;
    Ok(value
        .get("dist")
        .and_then(|dist| dist.get("cargo-dist-version"))
        .and_then(toml::Value::as_str)
        .map(str::to_string))
}

fn parse_root_cargo_dist_version(contents: &str) -> Result<Option<String>> {
    let value: toml::Value = toml::from_str(contents).context("Failed to parse Cargo.toml")?;
    Ok(value
        .get("workspace")
        .and_then(|workspace| workspace.get("metadata"))
        .and_then(|metadata| metadata.get("dist"))
        .and_then(|dist| dist.get("cargo-dist-version"))
        .and_then(toml::Value::as_str)
        .map(str::to_string))
}

fn ensure_dist_package_opt_in(checkout_dir: &Path) -> Result<()> {
    let manifest_path = find_binary_manifest(checkout_dir)?;
    let original = fs::read_to_string(&manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;
    let updated = ensure_dist_package_opt_in_text(&original);
    if updated != original {
        fs::write(&manifest_path, updated)
            .with_context(|| format!("Failed to write {}", manifest_path.display()))?;
    }
    Ok(())
}

fn find_binary_manifest(checkout_dir: &Path) -> Result<PathBuf> {
    let cli_manifest = checkout_dir.join("crates/keel-cli/Cargo.toml");
    if cli_manifest.exists() {
        return Ok(cli_manifest);
    }

    let root_manifest = checkout_dir.join("Cargo.toml");
    if root_manifest.exists() {
        return Ok(root_manifest);
    }

    bail!(
        "Could not find a Cargo manifest for the {APP_NAME} binary under {}",
        checkout_dir.display()
    )
}

fn ensure_dist_package_opt_in_text(contents: &str) -> String {
    const DIST_HEADER: &str = "[package.metadata.dist]";
    const DIST_FLAG: &str = "dist = true";

    if let Some(section_start) = contents.find(DIST_HEADER) {
        let body_start = section_start + DIST_HEADER.len();
        let section_end = contents[body_start..]
            .find("\n[")
            .map(|offset| body_start + offset)
            .unwrap_or(contents.len());
        let section_body = &contents[body_start..section_end];

        if section_body.lines().any(|line| line.trim() == DIST_FLAG) {
            return contents.to_string();
        }

        let mut updated = String::with_capacity(contents.len() + DIST_FLAG.len() + 1);
        updated.push_str(&contents[..body_start]);
        updated.push('\n');
        updated.push_str(DIST_FLAG);
        if !section_body.starts_with('\n') {
            updated.push('\n');
        }
        updated.push_str(section_body);
        updated.push_str(&contents[section_end..]);
        return updated;
    }

    let insert_at = contents
        .find("[package.metadata.deb]")
        .unwrap_or(contents.len());
    let mut updated =
        String::with_capacity(contents.len() + DIST_HEADER.len() + DIST_FLAG.len() + 4);
    updated.push_str(&contents[..insert_at]);
    if !updated.ends_with('\n') && !updated.is_empty() {
        updated.push('\n');
    }
    updated.push_str(DIST_HEADER);
    updated.push('\n');
    updated.push_str(DIST_FLAG);
    updated.push_str("\n\n");
    updated.push_str(&contents[insert_at..]);
    updated
}

fn ensure_cargo_dist(
    cache_root: &Path,
    version: &str,
    installer: InstallerPlatform,
) -> Result<PathBuf> {
    let install_root = cache_root
        .join("tooling")
        .join(format!("cargo-dist-v{version}"));
    let bin_name = if cfg!(windows) { "dist.exe" } else { "dist" };
    let dist_bin = install_root.join("bin").join(bin_name);
    if dist_bin.exists() {
        return Ok(dist_bin);
    }

    let temp_root = cache_root.join("tmp");
    fs::create_dir_all(&temp_root)
        .with_context(|| format!("Failed to create {}", temp_root.display()))?;
    let download_dir = Builder::new()
        .prefix("cargo-dist-installer-")
        .tempdir_in(&temp_root)
        .context("Failed to create a temporary cargo-dist install directory")?;
    let script_path = download_dir.path().join(installer.cargo_dist_script_name());
    let url = format!(
        "https://github.com/{CARGO_DIST_REPOSITORY_SLUG}/releases/download/v{version}/{}",
        installer.cargo_dist_script_name()
    );

    println!(
        "Installing cargo-dist {version} into {}...",
        install_root.display()
    );
    download_remote_script(&url, &script_path)?;
    run_cargo_dist_installer(&script_path, installer, &install_root)?;

    if !dist_bin.exists() {
        bail!(
            "cargo-dist installer completed without producing {}",
            dist_bin.display()
        );
    }

    Ok(dist_bin)
}

fn build_local_installer_bundle(
    checkout_dir: &Path,
    toolchain: &ToolchainRunner,
    dist_bin: &Path,
    host_target: &str,
    build_root: &Path,
) -> Result<PathBuf> {
    fs::create_dir_all(build_root)
        .with_context(|| format!("Failed to create {}", build_root.display()))?;
    let target_dir = build_root.join("target");
    fs::create_dir_all(&target_dir)
        .with_context(|| format!("Failed to create {}", target_dir.display()))?;

    let mut cmd = toolchain.command_for_tool(dist_bin);
    cmd.current_dir(checkout_dir)
        .env("CARGO_TARGET_DIR", &target_dir)
        .arg("build")
        .arg("--artifacts=all")
        .arg("--target")
        .arg(host_target)
        .arg("--allow-dirty");
    run_command(&mut cmd, "build a local dist installer bundle")?;

    let distrib_dir = target_dir.join("distrib");
    if !distrib_dir.exists() {
        bail!(
            "Local dist build completed without producing {}",
            distrib_dir.display()
        );
    }

    Ok(distrib_dir)
}

fn download_remote_script(url: &str, dest: &Path) -> Result<()> {
    #[cfg(windows)]
    {
        let mut cmd = Command::new("powershell");
        cmd.arg("-NoLogo")
            .arg("-NoProfile")
            .arg("-Command")
            .arg("Invoke-WebRequest -Uri $env:KEEL_UPGRADE_URL -OutFile $env:KEEL_UPGRADE_DEST")
            .env("KEEL_UPGRADE_URL", url)
            .env("KEEL_UPGRADE_DEST", dest);
        run_command(&mut cmd, &format!("download {url}"))?;
    }

    #[cfg(not(windows))]
    {
        let mut cmd = Command::new("curl");
        cmd.arg("--proto")
            .arg("=https")
            .arg("--tlsv1.2")
            .arg("-LsSf")
            .arg("-o")
            .arg(dest)
            .arg(url);
        run_command(&mut cmd, &format!("download {url}"))?;
    }

    Ok(())
}

fn run_installer_script(
    script_path: &Path,
    installer: InstallerPlatform,
    download_url: Option<&str>,
) -> Result<()> {
    let mut cmd = installer.command_for_script(script_path);
    if let Some(download_url) = download_url {
        cmd.env(installer.download_url_env, download_url);
    }
    run_command(&mut cmd, &format!("run {}", script_path.display()))
}

fn run_cargo_dist_installer(
    script_path: &Path,
    installer: InstallerPlatform,
    install_root: &Path,
) -> Result<()> {
    let mut cmd = installer.command_for_script(script_path);
    cmd.env("CARGO_HOME", install_root)
        .env("CARGO_DIST_NO_MODIFY_PATH", "1");
    run_command(
        &mut cmd,
        &format!("install cargo-dist via {}", script_path.display()),
    )
}

fn discover_supported_toolchain(checkout_dir: &Path) -> Result<ToolchainRunner> {
    let mut attempts = Vec::new();
    for candidate in ToolchainRunner::candidates(checkout_dir) {
        match candidate.supports_checkout(checkout_dir) {
            Ok(true) => return Ok(candidate),
            Ok(false) => attempts.push(format!("{} metadata probe failed", candidate.label)),
            Err(err) => attempts.push(format!("{} unavailable: {err}", candidate.label)),
        }
    }

    bail!(
        "No supported Rust toolchain found for building {APP_NAME}. Tried: {}",
        attempts.join("; ")
    )
}

fn detect_host_target(toolchain: &ToolchainRunner, checkout_dir: &Path) -> Result<String> {
    let mut cmd = toolchain.command_for_tool("rustc");
    cmd.current_dir(checkout_dir).arg("-vV");
    let output = read_stdout(&mut cmd, "inspect the active Rust host target")?;
    output
        .lines()
        .find_map(|line| line.strip_prefix("host: ").map(str::to_string))
        .ok_or_else(|| anyhow::anyhow!("Failed to parse `rustc -vV` host triple"))
}

fn file_url_for_dir(path: &Path) -> Result<String> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize {}", path.display()))?;

    #[cfg(windows)]
    let normalized = canonical.to_string_lossy().replace('\\', "/");
    #[cfg(not(windows))]
    let normalized = canonical.to_string_lossy().into_owned();

    let encoded = percent_encode_path(&normalized);
    #[cfg(windows)]
    {
        Ok(format!("file:///{encoded}"))
    }
    #[cfg(not(windows))]
    {
        Ok(format!("file://{encoded}"))
    }
}

fn percent_encode_path(path: &str) -> String {
    let mut encoded = String::with_capacity(path.len());
    for byte in path.bytes() {
        if matches!(
            byte,
            b'A'..=b'Z'
                | b'a'..=b'z'
                | b'0'..=b'9'
                | b'-'
                | b'.'
                | b'_'
                | b'~'
                | b'/'
                | b':'
        ) {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push_str(&format!("{byte:02X}"));
        }
    }
    encoded
}

fn github_repo_slug(repository: &str) -> Result<String> {
    let trimmed = repository.trim().trim_end_matches(".git");
    if let Some(slug) = trimmed.strip_prefix("https://github.com/") {
        return Ok(slug.to_string());
    }
    if let Some(slug) = trimmed.strip_prefix("http://github.com/") {
        return Ok(slug.to_string());
    }
    if let Some(slug) = trimmed.strip_prefix("git@github.com:") {
        return Ok(slug.to_string());
    }

    bail!("Unsupported GitHub repository URL: {repository}")
}

fn read_stdout(cmd: &mut Command, description: &str) -> Result<String> {
    let output = cmd
        .output()
        .with_context(|| format!("Failed to {description}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            bail!("{description} failed with exit status {}", output.status);
        }
        bail!(
            "{description} failed with exit status {}: {stderr}",
            output.status
        );
    }

    let stdout = String::from_utf8(output.stdout)
        .with_context(|| format!("{description} produced non-UTF-8 output"))?;
    Ok(stdout.trim().to_string())
}

fn run_command(cmd: &mut Command, description: &str) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("Failed to {description}"))?;
    if !status.success() {
        bail!("{description} failed with exit status {status}");
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct InstallerPlatform {
    script_name: &'static str,
    download_url_env: &'static str,
}

impl InstallerPlatform {
    fn current() -> Self {
        if cfg!(windows) {
            Self {
                script_name: "keel-installer.ps1",
                download_url_env: "KEEL_DOWNLOAD_URL",
            }
        } else {
            Self {
                script_name: "keel-installer.sh",
                download_url_env: "KEEL_DOWNLOAD_URL",
            }
        }
    }

    fn cargo_dist_script_name(self) -> &'static str {
        if cfg!(windows) {
            "cargo-dist-installer.ps1"
        } else {
            "cargo-dist-installer.sh"
        }
    }

    fn command_for_script(self, script_path: &Path) -> Command {
        if cfg!(windows) {
            let mut cmd = Command::new("powershell");
            cmd.arg("-NoLogo")
                .arg("-NoProfile")
                .arg("-ExecutionPolicy")
                .arg("Bypass")
                .arg("-File")
                .arg(script_path);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.arg(script_path);
            cmd
        }
    }
}

#[derive(Clone, Debug)]
struct ToolchainRunner {
    label: &'static str,
    prefix: Vec<OsString>,
}

impl ToolchainRunner {
    fn candidates(checkout_dir: &Path) -> Vec<Self> {
        let mut candidates = vec![Self::system()];
        if command_exists("rustup") {
            candidates.push(Self::rustup_stable());
        }
        if checkout_dir.join("flake.nix").exists() && command_exists("nix") {
            candidates.push(Self::nix_develop());
        }
        candidates
    }

    fn system() -> Self {
        Self {
            label: "system cargo/rustc",
            prefix: Vec::new(),
        }
    }

    fn rustup_stable() -> Self {
        Self {
            label: "rustup stable",
            prefix: vec![
                OsString::from("rustup"),
                OsString::from("run"),
                OsString::from("stable"),
            ],
        }
    }

    fn nix_develop() -> Self {
        Self {
            label: "nix develop",
            prefix: vec![
                OsString::from("nix"),
                OsString::from("develop"),
                OsString::from("--command"),
            ],
        }
    }

    fn command_for_tool<T: AsRef<OsStr>>(&self, tool: T) -> Command {
        if self.prefix.is_empty() {
            return Command::new(tool);
        }

        let mut cmd = Command::new(&self.prefix[0]);
        cmd.args(&self.prefix[1..]).arg(tool);
        cmd
    }

    fn supports_checkout(&self, checkout_dir: &Path) -> Result<bool> {
        let manifest_path = checkout_dir.join("Cargo.toml");
        let mut cmd = self.command_for_tool("cargo");
        cmd.current_dir(checkout_dir)
            .arg("metadata")
            .arg("--format-version")
            .arg("1")
            .arg("--locked")
            .arg("--manifest-path")
            .arg(&manifest_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let status = cmd.status()?;
        Ok(status.success())
    }
}

fn command_exists(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

struct CachedWorktree {
    repo_dir: PathBuf,
    path: PathBuf,
}

impl CachedWorktree {
    fn checkout(repo_dir: &Path, resolved: &str, cache_root: &Path) -> Result<Self> {
        let checkout_root = cache_root.join("checkouts");
        fs::create_dir_all(&checkout_root)
            .with_context(|| format!("Failed to create {}", checkout_root.display()))?;

        let tempdir = Builder::new()
            .prefix("checkout-")
            .tempdir_in(&checkout_root)
            .context("Failed to create a temporary checkout directory")?;
        let path = tempdir.path().to_path_buf();
        std::mem::forget(tempdir);

        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(repo_dir)
            .arg("worktree")
            .arg("add")
            .arg("--detach")
            .arg(&path)
            .arg(resolved);
        if let Err(err) = run_command(&mut cmd, "create a cached source worktree") {
            let _ = fs::remove_dir_all(&path);
            return Err(err);
        }

        Ok(Self {
            repo_dir: repo_dir.to_path_buf(),
            path,
        })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for CachedWorktree {
    fn drop(&mut self) {
        let _ = Command::new("git")
            .arg("-C")
            .arg(&self.repo_dir)
            .arg("worktree")
            .arg("remove")
            .arg("--force")
            .arg(&self.path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dist_workspace_version_reads_dist_table() {
        let version = parse_dist_workspace_version(
            r#"
[dist]
cargo-dist-version = "0.30.3"
"#,
        )
        .unwrap();
        assert_eq!(version.as_deref(), Some("0.30.3"));
    }

    #[test]
    fn parse_root_cargo_dist_version_reads_workspace_metadata() {
        let version = parse_root_cargo_dist_version(
            r#"
[workspace.metadata.dist]
cargo-dist-version = "0.22.1"
"#,
        )
        .unwrap();
        assert_eq!(version.as_deref(), Some("0.22.1"));
    }

    #[test]
    fn ensure_dist_package_opt_in_adds_missing_section() {
        let input = r#"[package]
name = "keel"

[package.metadata.deb]
depends = "$auto"
"#;
        let output = ensure_dist_package_opt_in_text(input);
        assert!(output.contains("[package.metadata.dist]"));
        assert!(output.contains("dist = true"));
        assert!(output.contains("[package.metadata.deb]"));
    }

    #[test]
    fn ensure_dist_package_opt_in_updates_existing_section() {
        let input = r#"[package]
name = "keel"

[package.metadata.dist]
include = ["README.md"]

[package.metadata.deb]
depends = "$auto"
"#;
        let output = ensure_dist_package_opt_in_text(input);
        assert!(output.contains("[package.metadata.dist]\ndist = true\ninclude = [\"README.md\"]"));
    }

    #[test]
    fn ensure_dist_package_opt_in_preserves_existing_true() {
        let input = r#"[package.metadata.dist]
dist = true
"#;
        assert_eq!(ensure_dist_package_opt_in_text(input), input);
    }

    #[test]
    fn github_repo_slug_supports_https_and_ssh_forms() {
        assert_eq!(
            github_repo_slug("https://github.com/spoke-sh/keel.git").unwrap(),
            "spoke-sh/keel"
        );
        assert_eq!(
            github_repo_slug("git@github.com:spoke-sh/keel.git").unwrap(),
            "spoke-sh/keel"
        );
    }

    #[test]
    fn percent_encode_path_escapes_spaces() {
        assert_eq!(
            percent_encode_path("/tmp/keel upgrade/distrib"),
            "/tmp/keel%20upgrade/distrib"
        );
    }
}
