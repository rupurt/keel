//! Authentication commands for Hub-backed Keeper sessions.

use std::io::{self, Read};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use clap::Subcommand;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use keel::infrastructure::config::{self, Config, ConfigSource};

#[derive(Subcommand, Debug)]
pub enum AuthAction {
    /// Sign in through a Spoke Hub account and persist the session locally
    Login {
        /// Hub account email address
        #[arg(long)]
        email: String,
        /// Password for the Hub credential account
        #[arg(long, conflicts_with = "password_stdin")]
        password: Option<String>,
        /// Read the password from stdin
        #[arg(long, conflicts_with = "password")]
        password_stdin: bool,
        /// Explicit Hub base URL (overrides config)
        #[arg(long, value_name = "URL")]
        hub_url: Option<String>,
        /// Explicit path for the saved auth session file
        #[arg(long, value_name = "PATH")]
        session_file: Option<PathBuf>,
    },
    /// Show the current saved auth session
    Info {
        /// Output as JSON for scripting
        #[arg(long)]
        json: bool,
        /// Explicit path for the saved auth session file
        #[arg(long, value_name = "PATH")]
        session_file: Option<PathBuf>,
    },
    /// Revoke the current Hub session and remove the local session file
    Logout {
        /// Explicit path for the saved auth session file
        #[arg(long, value_name = "PATH")]
        session_file: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct AuthSessionSummary {
    session_file: String,
    issuer: String,
    provider: String,
    account_provider: Option<String>,
    account_id: Option<String>,
    provider_subject: String,
    role: String,
    session_id: Option<String>,
    scopes: Vec<String>,
    authenticated_at: String,
    hub_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct LogoutSummary {
    session_file: String,
    deleted_local_session: bool,
    hub_status: String,
}

#[derive(Debug, Deserialize)]
struct HubLoginResponse {
    provider: String,
    access_token: String,
    token_type: String,
    expires_in: u64,
    session_id: String,
    #[serde(default)]
    scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct HubAccountResponse {
    account_id: String,
    provider_subject: String,
    provider: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RevokeStatus {
    Revoked,
    AlreadyRevoked,
    Missing,
    Unauthorized,
    LocalOnly,
}

impl RevokeStatus {
    fn label(&self) -> &'static str {
        match self {
            RevokeStatus::Revoked => "revoked",
            RevokeStatus::AlreadyRevoked => "already-revoked",
            RevokeStatus::Missing => "missing-on-hub",
            RevokeStatus::Unauthorized => "unauthorized-on-hub",
            RevokeStatus::LocalOnly => "local-only",
        }
    }
}

struct HubClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl HubClient {
    fn new(base_url: &str) -> Result<Self> {
        Ok(Self {
            base_url: normalize_base_url(base_url),
            client: reqwest::blocking::Client::builder()
                .build()
                .context("Failed to build Hub HTTP client")?,
        })
    }

    fn login(&self, email: &str, password: &str) -> Result<HubLoginResponse> {
        let response = self
            .client
            .post(self.endpoint("/auth/login"))
            .json(&serde_json::json!({
                "email": email,
                "password": password,
            }))
            .send()
            .context("Failed to contact Hub login endpoint")?;

        parse_json_response(response, "Hub login")
    }

    fn me(&self, access_token: &str) -> Result<HubAccountResponse> {
        let response = self
            .client
            .get(self.endpoint("/protected/me"))
            .bearer_auth(access_token)
            .send()
            .context("Failed to contact Hub identity endpoint")?;

        parse_json_response(response, "Hub identity lookup")
    }

    fn revoke_session(&self, access_token: &str, session_id: &str) -> Result<RevokeStatus> {
        let response = self
            .client
            .post(self.endpoint(&format!("/sessions/{session_id}/revoke")))
            .bearer_auth(access_token)
            .json(&serde_json::json!({ "reason": "logout" }))
            .send()
            .context("Failed to contact Hub revoke endpoint")?;

        match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(RevokeStatus::Revoked),
            reqwest::StatusCode::CONFLICT => Ok(RevokeStatus::AlreadyRevoked),
            reqwest::StatusCode::NOT_FOUND => Ok(RevokeStatus::Missing),
            reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN => {
                Ok(RevokeStatus::Unauthorized)
            }
            _ => Err(response_error(response, "Hub session revoke")),
        }
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }
}

/// Run an auth action through the auth interface adapter.
pub fn run(action: AuthAction) -> Result<()> {
    match action {
        AuthAction::Login {
            email,
            password,
            password_stdin,
            hub_url,
            session_file,
        } => {
            let password = read_password(password, password_stdin)?;
            let summary = login_with_password(
                &email,
                &password,
                hub_url.as_deref(),
                session_file.as_deref(),
            )?;
            print!("{}", render_auth_summary("Logged in", &summary));
            Ok(())
        }
        AuthAction::Info { json, session_file } => {
            let summary = inspect_saved_session(session_file.as_deref())?;
            if json {
                println!("{}", serde_json::to_string_pretty(&summary)?);
            } else {
                print!("{}", render_auth_summary("Current auth session", &summary));
            }
            Ok(())
        }
        AuthAction::Logout { session_file } => {
            let summary = logout_saved_session(session_file.as_deref())?;
            print!("{}", render_logout_summary(&summary));
            Ok(())
        }
    }
}

fn login_with_password(
    email: &str,
    password: &str,
    hub_url_override: Option<&str>,
    session_file_override: Option<&Path>,
) -> Result<AuthSessionSummary> {
    let (config, source) = config::load_config();
    let hub_base_url = resolve_hub_base_url(&config, hub_url_override)?;
    let session_file = resolve_session_file(&config, &source, session_file_override)?;
    let client = HubClient::new(&hub_base_url)?;
    let login = client.login(email, password)?;
    let account = client.me(&login.access_token)?;

    let record = spoke_auth::AuthSessionRecord {
        authority: spoke_auth::AuthSessionAuthority {
            issuer: hub_base_url,
            provider: "hub".to_string(),
            account_provider: Some(login.provider),
        },
        subject: spoke_auth::AuthSessionSubject {
            account_id: Some(account.account_id.clone()),
            provider_subject: account.provider_subject.clone(),
            role: spoke_auth::derive_role_from_scopes(&login.scopes),
        },
        credential: spoke_auth::AuthSessionCredential {
            access_token: login.access_token,
            token_type: login.token_type,
            expires_at: spoke_auth::compute_expiry(login.expires_in),
        },
        session: spoke_auth::AuthSessionEnvelope {
            session_id: Some(login.session_id),
            scopes: login.scopes,
        },
        authenticated_at: chrono::Utc::now(),
    };
    spoke_auth::save_session_record(&session_file, &record)?;

    Ok(build_session_summary(&record, &session_file, "verified"))
}

fn inspect_saved_session(session_file_override: Option<&Path>) -> Result<AuthSessionSummary> {
    let (config, source) = config::load_config();
    let session_file = resolve_session_file(&config, &source, session_file_override)?;
    let record = spoke_auth::load_session_record(&session_file)?;
    let hub_status =
        match HubClient::new(&record.authority.issuer)?.me(&record.credential.access_token) {
            Ok(account) => {
                if account.provider
                    == record
                        .authority
                        .account_provider
                        .clone()
                        .unwrap_or_default()
                    && account.account_id == record.subject.account_id.clone().unwrap_or_default()
                    && account.provider_subject == record.subject.provider_subject
                {
                    "verified".to_string()
                } else {
                    "verified-with-drift".to_string()
                }
            }
            Err(error) => format!("unverified: {error}"),
        };

    Ok(build_session_summary(&record, &session_file, &hub_status))
}

fn logout_saved_session(session_file_override: Option<&Path>) -> Result<LogoutSummary> {
    let (config, source) = config::load_config();
    let session_file = resolve_session_file(&config, &source, session_file_override)?;
    let Some(record) = spoke_auth::load_session_record_if_exists(&session_file)? else {
        return Ok(LogoutSummary {
            session_file: session_file.display().to_string(),
            deleted_local_session: false,
            hub_status: "no-saved-session".to_string(),
        });
    };

    let revoke_status = match record.session.session_id.as_deref() {
        Some(session_id) => HubClient::new(&record.authority.issuer)?
            .revoke_session(&record.credential.access_token, session_id)?,
        None => RevokeStatus::LocalOnly,
    };

    let deleted_local_session = spoke_auth::delete_session_record(&session_file)?;
    Ok(LogoutSummary {
        session_file: session_file.display().to_string(),
        deleted_local_session,
        hub_status: revoke_status.label().to_string(),
    })
}

fn build_session_summary(
    record: &spoke_auth::AuthSessionRecord,
    session_file: &Path,
    hub_status: &str,
) -> AuthSessionSummary {
    AuthSessionSummary {
        session_file: session_file.display().to_string(),
        issuer: record.authority.issuer.clone(),
        provider: record.authority.provider.clone(),
        account_provider: record.authority.account_provider.clone(),
        account_id: record.subject.account_id.clone(),
        provider_subject: record.subject.provider_subject.clone(),
        role: record.subject.role.clone(),
        session_id: record.session.session_id.clone(),
        scopes: record.session.scopes.clone(),
        authenticated_at: record.authenticated_at.to_rfc3339(),
        hub_status: hub_status.to_string(),
    }
}

fn render_auth_summary(title: &str, summary: &AuthSessionSummary) -> String {
    let mut lines = Vec::new();
    lines.push(format!("{title}:"));
    lines.push(format!("  session_file = \"{}\"", summary.session_file));
    lines.push(format!("  issuer = \"{}\"", summary.issuer));
    lines.push(format!("  provider = \"{}\"", summary.provider));
    if let Some(account_provider) = &summary.account_provider {
        lines.push(format!("  account_provider = \"{}\"", account_provider));
    }
    if let Some(account_id) = &summary.account_id {
        lines.push(format!("  account_id = \"{}\"", account_id));
    }
    lines.push(format!(
        "  provider_subject = \"{}\"",
        summary.provider_subject
    ));
    lines.push(format!("  role = \"{}\"", summary.role));
    if let Some(session_id) = &summary.session_id {
        lines.push(format!("  session_id = \"{}\"", session_id));
    }
    lines.push(format!("  scopes = {:?}", summary.scopes));
    lines.push(format!(
        "  authenticated_at = \"{}\"",
        summary.authenticated_at
    ));
    lines.push(format!("  hub_status = \"{}\"", summary.hub_status));
    lines.push(String::new());
    lines.join("\n")
}

fn render_logout_summary(summary: &LogoutSummary) -> String {
    let mut lines = Vec::new();
    lines.push("Logged out:".to_string());
    lines.push(format!("  session_file = \"{}\"", summary.session_file));
    lines.push(format!(
        "  deleted_local_session = {}",
        summary.deleted_local_session
    ));
    lines.push(format!("  hub_status = \"{}\"", summary.hub_status));
    lines.push(String::new());
    lines.join("\n")
}

fn read_password(password: Option<String>, password_stdin: bool) -> Result<String> {
    match (password, password_stdin) {
        (Some(password), false) => Ok(password),
        (None, true) => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read password from stdin")?;
            let password = buffer.trim_end_matches(['\r', '\n']).to_string();
            if password.is_empty() {
                bail!("Password read from stdin was empty");
            }
            Ok(password)
        }
        (None, false) => {
            bail!("Provide Hub credentials with `--password` or `--password-stdin`")
        }
        (Some(_), true) => bail!("Use either `--password` or `--password-stdin`, not both"),
    }
}

fn resolve_hub_base_url(config: &Config, hub_url_override: Option<&str>) -> Result<String> {
    if let Some(url) = hub_url_override {
        return Ok(normalize_base_url(url));
    }

    if let Some(url) = config.storage.server.hub_base_url.as_deref() {
        return Ok(normalize_base_url(url));
    }

    bail!(
        "Hub base URL is not configured. Set `[storage.server].hub_base_url` in `keel.toml` or pass `--hub-url`."
    )
}

fn resolve_session_file(
    config: &Config,
    source: &ConfigSource,
    session_file_override: Option<&Path>,
) -> Result<PathBuf> {
    let configured_session_file = config
        .auth
        .session_file
        .as_deref()
        .map(|value| config::resolve_path_from_source(source, value));
    spoke_auth::resolve_session_file_path(session_file_override, configured_session_file.as_deref())
}

fn normalize_base_url(base_url: &str) -> String {
    base_url.trim_end_matches('/').to_string()
}

fn parse_json_response<T>(response: reqwest::blocking::Response, operation: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    if !response.status().is_success() {
        return Err(response_error(response, operation));
    }

    response
        .json::<T>()
        .with_context(|| format!("Failed to decode {operation} response"))
}

fn response_error(response: reqwest::blocking::Response, operation: &str) -> anyhow::Error {
    let status = response.status();
    let detail = response
        .text()
        .ok()
        .map(|body| body.trim().to_string())
        .filter(|body| !body.is_empty())
        .unwrap_or_else(|| status.to_string());
    anyhow!("{operation} failed ({status}): {detail}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use tempfile::TempDir;

    #[derive(Clone)]
    struct MockExchange {
        method: &'static str,
        path: &'static str,
        auth_header: Option<&'static str>,
        body_contains: &'static [&'static str],
        status_line: &'static str,
        response_body: &'static str,
    }

    fn spawn_mock_hub(exchanges: Vec<MockExchange>) -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            for exchange in exchanges {
                let (mut stream, _) = listener.accept().unwrap();
                let request = read_request(&mut stream);
                assert!(
                    request.starts_with(&format!("{} {} ", exchange.method, exchange.path)),
                    "unexpected request line: {request:?}"
                );
                if let Some(auth_header) = exchange.auth_header {
                    assert!(
                        request
                            .to_ascii_lowercase()
                            .contains(&auth_header.to_ascii_lowercase()),
                        "missing auth header {auth_header} in request {request:?}"
                    );
                }
                for needle in exchange.body_contains {
                    assert!(
                        request.contains(needle),
                        "missing body fragment {needle} in request {request:?}"
                    );
                }

                let response = format!(
                    "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{}",
                    exchange.status_line,
                    exchange.response_body.len(),
                    exchange.response_body
                );
                stream.write_all(response.as_bytes()).unwrap();
            }
        });

        (format!("http://{}", addr), handle)
    }

    fn read_request(stream: &mut TcpStream) -> String {
        let mut buffer = Vec::new();
        let mut chunk = [0_u8; 1024];
        let mut header_end = None;
        let mut content_length = 0_usize;

        loop {
            let read = stream.read(&mut chunk).unwrap();
            if read == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..read]);

            if header_end.is_none()
                && let Some(position) = find_bytes(&buffer, b"\r\n\r\n")
            {
                header_end = Some(position + 4);
                let headers = String::from_utf8_lossy(&buffer[..position + 4]);
                content_length = headers
                    .lines()
                    .find_map(|line| {
                        line.strip_prefix("Content-Length: ")
                            .or_else(|| line.strip_prefix("content-length: "))
                    })
                    .and_then(|value| value.trim().parse::<usize>().ok())
                    .unwrap_or(0);
            }

            if let Some(header_end) = header_end
                && buffer.len() >= header_end + content_length
            {
                break;
            }
        }

        String::from_utf8(buffer).unwrap()
    }

    fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
    }

    fn sample_session(issuer: &str) -> spoke_auth::AuthSessionRecord {
        spoke_auth::AuthSessionRecord {
            authority: spoke_auth::AuthSessionAuthority {
                issuer: issuer.to_string(),
                provider: "hub".to_string(),
                account_provider: Some("credentials".to_string()),
            },
            subject: spoke_auth::AuthSessionSubject {
                account_id: Some("acct-123".to_string()),
                provider_subject: "credentials:pilot@spoke.sh".to_string(),
                role: "admin".to_string(),
            },
            credential: spoke_auth::AuthSessionCredential {
                access_token: "super-secret-token".to_string(),
                token_type: "Bearer".to_string(),
                expires_at: None,
            },
            session: spoke_auth::AuthSessionEnvelope {
                session_id: Some("session-123".to_string()),
                scopes: vec!["admin".to_string()],
            },
            authenticated_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn login_persists_session_and_redacts_human_output() {
        let (hub_url, server) = spawn_mock_hub(vec![
            MockExchange {
                method: "POST",
                path: "/auth/login",
                auth_header: None,
                body_contains: &[
                    "\"email\":\"pilot@spoke.sh\"",
                    "\"password\":\"broadcast123\"",
                ],
                status_line: "200 OK",
                response_body: r#"{"provider":"credentials","access_token":"super-secret-token","token_type":"Bearer","expires_in":900,"session_id":"session-123","scopes":["admin"]}"#,
            },
            MockExchange {
                method: "GET",
                path: "/protected/me",
                auth_header: Some("authorization: bearer super-secret-token"),
                body_contains: &[],
                status_line: "200 OK",
                response_body: r#"{"account_id":"acct-123","provider_subject":"credentials:pilot@spoke.sh","provider":"credentials"}"#,
            },
        ]);
        let temp = TempDir::new().unwrap();
        let session_file = temp.path().join("auth").join("session.json");

        let summary = login_with_password(
            "pilot@spoke.sh",
            "broadcast123",
            Some(&hub_url),
            Some(&session_file),
        )
        .unwrap();
        server.join().unwrap();

        let saved = spoke_auth::load_session_record(&session_file).unwrap();
        assert_eq!(saved.subject.provider_subject, "credentials:pilot@spoke.sh");
        assert_eq!(
            saved.authority.account_provider.as_deref(),
            Some("credentials")
        );
        assert_eq!(summary.hub_status, "verified");

        let rendered = render_auth_summary("Logged in", &summary);
        assert!(!rendered.contains("super-secret-token"));
        assert!(rendered.contains("provider_subject = \"credentials:pilot@spoke.sh\""));
    }

    #[test]
    fn info_verifies_saved_session_without_printing_token() {
        let (hub_url, server) = spawn_mock_hub(vec![MockExchange {
            method: "GET",
            path: "/protected/me",
            auth_header: Some("authorization: bearer super-secret-token"),
            body_contains: &[],
            status_line: "200 OK",
            response_body: r#"{"account_id":"acct-123","provider_subject":"credentials:pilot@spoke.sh","provider":"credentials"}"#,
        }]);
        let temp = TempDir::new().unwrap();
        let session_file = temp.path().join("session.json");
        spoke_auth::save_session_record(&session_file, &sample_session(&hub_url)).unwrap();

        let summary = inspect_saved_session(Some(&session_file)).unwrap();
        server.join().unwrap();

        assert_eq!(summary.hub_status, "verified");
        let rendered = render_auth_summary("Current auth session", &summary);
        assert!(!rendered.contains("super-secret-token"));
        assert!(rendered.contains("hub_status = \"verified\""));
    }

    #[test]
    fn logout_revokes_remote_session_and_deletes_saved_file() {
        let (hub_url, server) = spawn_mock_hub(vec![MockExchange {
            method: "POST",
            path: "/sessions/session-123/revoke",
            auth_header: Some("authorization: bearer super-secret-token"),
            body_contains: &["\"reason\":\"logout\""],
            status_line: "204 No Content",
            response_body: "",
        }]);
        let temp = TempDir::new().unwrap();
        let session_file = temp.path().join("session.json");
        spoke_auth::save_session_record(&session_file, &sample_session(&hub_url)).unwrap();

        let summary = logout_saved_session(Some(&session_file)).unwrap();
        server.join().unwrap();

        assert_eq!(summary.hub_status, "revoked");
        assert!(summary.deleted_local_session);
        assert!(!session_file.exists());
    }
}
