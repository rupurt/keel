//! Spoke Auth
//!
//! Provides shared session persistence and execution-context resolution.

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum Actor {
    LocalSystem { os_user: String },
    Authenticated { identity: String, role: String },
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub actor: Actor,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthSessionAuthority {
    pub issuer: String,
    pub provider: String,
    #[serde(default)]
    pub account_provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthSessionSubject {
    #[serde(default)]
    pub account_id: Option<String>,
    pub provider_subject: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthSessionCredential {
    pub access_token: String,
    pub token_type: String,
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthSessionEnvelope {
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthSessionRecord {
    pub authority: AuthSessionAuthority,
    pub subject: AuthSessionSubject,
    pub credential: AuthSessionCredential,
    pub session: AuthSessionEnvelope,
    pub authenticated_at: DateTime<Utc>,
}

impl ExecutionContext {
    pub fn new_local(os_user: String) -> Self {
        Self {
            actor: Actor::LocalSystem { os_user },
            timestamp: Utc::now(),
        }
    }
}

pub fn default_session_file() -> Result<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        return Ok(config_dir.join("keel").join("auth-session.json"));
    }

    Ok(PathBuf::from(".keel-auth-session.json"))
}

pub fn resolve_session_file_path(
    explicit_path: Option<&Path>,
    configured_path: Option<&Path>,
) -> Result<PathBuf> {
    if let Some(path) = explicit_path {
        return Ok(path.to_path_buf());
    }
    if let Some(path) = configured_path {
        return Ok(path.to_path_buf());
    }
    default_session_file()
}

pub fn derive_role_from_scopes(scopes: &[String]) -> String {
    if scopes
        .iter()
        .any(|scope| scope == "admin" || scope.ends_with(":admin"))
    {
        "admin".to_string()
    } else {
        "member".to_string()
    }
}

pub fn compute_expiry(expires_in_seconds: u64) -> Option<DateTime<Utc>> {
    let seconds = i64::try_from(expires_in_seconds).ok()?;
    Some(Utc::now() + Duration::seconds(seconds))
}

pub fn load_session_record(path: &Path) -> Result<AuthSessionRecord> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read auth session file: {}", path.display()))?;
    serde_json::from_str::<AuthSessionRecord>(&content).with_context(|| {
        format!(
            "Failed to parse auth session file as a stored Hub session: {}",
            path.display()
        )
    })
}

pub fn load_session_record_if_exists(path: &Path) -> Result<Option<AuthSessionRecord>> {
    if !path.exists() {
        return Ok(None);
    }

    load_session_record(path).map(Some)
}

pub fn save_session_record(path: &Path, session: &AuthSessionRecord) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create auth session directory: {}",
                parent.display()
            )
        })?;
    }

    let content =
        serde_json::to_string_pretty(session).context("Failed to serialize auth session record")?;
    fs::write(path, content)
        .with_context(|| format!("Failed to write auth session file: {}", path.display()))
}

pub fn delete_session_record(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    fs::remove_file(path)
        .with_context(|| format!("Failed to delete auth session file: {}", path.display()))?;
    Ok(true)
}

pub fn load_auth_context(
    explicit_path: Option<&Path>,
    configured_path: Option<&Path>,
) -> Result<ExecutionContext> {
    if let Some(path) = explicit_path {
        return load_auth_context_from_file(path, true);
    }

    let session_path = resolve_session_file_path(None, configured_path)?;
    if session_path.exists() {
        return load_auth_context_from_file(&session_path, false);
    }

    let os_user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    Ok(ExecutionContext::new_local(os_user))
}

fn load_auth_context_from_file(path: &Path, allow_legacy_token: bool) -> Result<ExecutionContext> {
    match load_session_record(path) {
        Ok(session) => Ok(ExecutionContext {
            actor: Actor::Authenticated {
                identity: session.subject.provider_subject,
                role: session.subject.role,
            },
            timestamp: Utc::now(),
        }),
        Err(session_error) if allow_legacy_token => {
            let token = fs::read_to_string(path)
                .with_context(|| format!("Failed to read auth file: {}", path.display()))?;
            if token.trim().is_empty() {
                return Err(anyhow!("Auth file is empty: {}", path.display()));
            }

            Ok(ExecutionContext {
                actor: Actor::Authenticated {
                    identity: "agent".to_string(),
                    role: "system".to_string(),
                },
                timestamp: Utc::now(),
            })
        }
        Err(session_error) => Err(session_error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn sample_session() -> AuthSessionRecord {
        AuthSessionRecord {
            authority: AuthSessionAuthority {
                issuer: "https://hub.spoke.test".to_string(),
                provider: "hub".to_string(),
                account_provider: Some("credentials".to_string()),
            },
            subject: AuthSessionSubject {
                account_id: Some("acct-123".to_string()),
                provider_subject: "credentials:pilot@spoke.sh".to_string(),
                role: "admin".to_string(),
            },
            credential: AuthSessionCredential {
                access_token: "secret-token".to_string(),
                token_type: "Bearer".to_string(),
                expires_at: Some(Utc::now() + Duration::seconds(900)),
            },
            session: AuthSessionEnvelope {
                session_id: Some("session-123".to_string()),
                scopes: vec!["admin".to_string()],
            },
            authenticated_at: Utc::now(),
        }
    }

    #[test]
    fn session_record_round_trips_without_losing_provider_neutral_shape() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("auth").join("session.json");
        let session = sample_session();

        save_session_record(&path, &session).unwrap();
        let loaded = load_session_record(&path).unwrap();

        assert_eq!(loaded, session);
        assert_eq!(loaded.authority.provider, "hub");
        assert_eq!(
            loaded.authority.account_provider.as_deref(),
            Some("credentials")
        );
        assert_eq!(
            loaded.subject.provider_subject,
            "credentials:pilot@spoke.sh"
        );
        assert_eq!(loaded.session.session_id.as_deref(), Some("session-123"));
    }

    #[test]
    fn load_auth_context_uses_saved_session_when_present() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("session.json");
        save_session_record(&path, &sample_session()).unwrap();

        let ctx = load_auth_context(Some(&path), None).unwrap();
        assert_eq!(
            ctx.actor,
            Actor::Authenticated {
                identity: "credentials:pilot@spoke.sh".to_string(),
                role: "admin".to_string(),
            }
        );
    }

    #[test]
    fn load_auth_context_falls_back_to_legacy_explicit_token_file() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("legacy.jwt");
        fs::write(&path, "signed-jwt").unwrap();

        let ctx = load_auth_context(Some(&path), None).unwrap();
        assert_eq!(
            ctx.actor,
            Actor::Authenticated {
                identity: "agent".to_string(),
                role: "system".to_string(),
            }
        );
    }

    #[test]
    fn load_auth_context_returns_local_system_when_no_session_exists() {
        let temp = TempDir::new().unwrap();
        let missing = temp.path().join("missing-session.json");

        let ctx = load_auth_context(None, Some(&missing)).unwrap();
        assert!(matches!(ctx.actor, Actor::LocalSystem { .. }));
    }

    #[test]
    fn derive_role_from_scopes_defaults_to_member() {
        assert_eq!(derive_role_from_scopes(&[]), "member");
        assert_eq!(
            derive_role_from_scopes(&["projects:admin".to_string()]),
            "admin"
        );
    }
}
