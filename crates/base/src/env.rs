use std::env;

use crate::{Error, Result};

pub fn get_env(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn get_env_or(key: &str, default: &str) -> String {
    get_env(key).unwrap_or_else(|| default.to_string())
}

pub fn get_required_env(key: &str) -> Result<String> {
    get_env(key)
        .ok_or_else(|| Error::NotFound(format!("Required environment variable {} is not set", key)))
}

pub fn get_port() -> u16 {
    get_env_or("PORT", "80").parse().unwrap_or(80)
}

pub fn get_port_or(default: u16) -> u16 {
    match get_env("PORT").and_then(|v| v.parse().ok()) {
        Some(port) => port,
        None => default,
    }
}

pub fn get_database_url() -> Option<String> {
    get_env("DATABASE_URL")
}

pub fn is_development() -> bool {
    is_environment("development")
}

pub fn is_staging() -> bool {
    is_environment("staging")
}

pub fn is_production() -> bool {
    is_environment("production")
}

pub fn is_environment(environment: &str) -> bool {
    !environment.is_empty()
        && get_env("NODE_ENV").map(|v| v.to_lowercase()) == Some(environment.to_lowercase())
}

pub fn is_any_environment(environments: &[&str]) -> bool {
    environments.iter().any(|e| is_environment(e))
}

pub fn get_allow_anonymous_users() -> bool {
    get_env("ALLOW_ANONYMOUS_USERS") != Some("false".to_string())
}
