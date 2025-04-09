//! A cross-platform environment variable expander that supports Unix-style (`$VAR`, `${VAR}`)
//! and Windows-style (`%VAR%`) syntax.

use std::env;

use regex::Regex;

use std::fmt;

/// Custom error type for environment variable expansion.
#[derive(Debug)]
pub enum EnvExpansionError {
    MissingVar(String),
}

impl fmt::Display for EnvExpansionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvExpansionError::MissingVar(var) => {
                write!(f, "Missing environment variable: {}", var)
            }
        }
    }
}

impl std::error::Error for EnvExpansionError {}

/// Expands environment variable placeholders in a string with actual environment values.
///
/// - On **Unix**, supports `$VAR` and `${VAR}`.
/// - On **Windows**, supports `%VAR%`.
///
/// # Errors
///
/// Currently, missing variables are replaced with an empty string.
/// A stricter mode can be implemented later to return an error for missing variables.
///
pub fn expand_env_vars(input: &str) -> Result<String, EnvExpansionError> {
    #[cfg(unix)]
    {
        let unix_re = Regex::new(r"\$(\w+)|\$\{(\w+)\}").unwrap();
        let result = unix_re.replace_all(input, |caps: &regex::Captures| {
            let var_name = caps
                .get(1)
                .or_else(|| caps.get(2))
                .map(|m| m.as_str())
                .unwrap_or("");
            env::var(var_name).unwrap_or_default()
        });
        Ok(result.into_owned())
    }

    #[cfg(windows)]
    {
        let windows_re = Regex::new(r"%(\w+)%").unwrap();
        let result = windows_re.replace_all(input, |caps: &regex::Captures| {
            let var_name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            env::var(var_name).unwrap_or_default()
        });
        result.into_owned()
    }
}
