//! Spoke Auth
//!
//! Provides authentication parsing and validation logic.

use anyhow::Result;
use chrono::{DateTime, Utc};
use std::path::Path;

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

impl ExecutionContext {
    pub fn new_local(os_user: String) -> Self {
        Self {
            actor: Actor::LocalSystem { os_user },
            timestamp: Utc::now(),
        }
    }
}

pub fn load_auth_context(token_path: Option<&Path>) -> Result<ExecutionContext> {
    if let Some(_path) = token_path {
        // Placeholder for real JWT loading
        Ok(ExecutionContext {
            actor: Actor::Authenticated {
                identity: "agent".to_string(),
                role: "system".to_string(),
            },
            timestamp: Utc::now(),
        })
    } else {
        let os_user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        Ok(ExecutionContext::new_local(os_user))
    }
}
