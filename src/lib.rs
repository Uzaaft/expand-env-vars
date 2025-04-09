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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_var_unix() {
        unsafe {
            std::env::set_var("USER", "alice");
        }
        let input = "Hello $USER!";
        let output = expand_env_vars(input).unwrap();
        assert_eq!(output, "Hello alice!");
    }

    #[test]
    fn test_braced_var_unix() {
        unsafe {
            std::env::set_var("HOME", "/home/alice");
        }
        let input = "Path: ${HOME}/code";
        let output = expand_env_vars(input).unwrap();
        assert_eq!(output, "Path: /home/alice/code");
    }

    #[test]
    fn test_multiple_vars_unix() {
        unsafe {
            std::env::set_var("USER", "bob");
            std::env::set_var("SHELL", "/bin/bash");
        }
        let input = "$USER uses $SHELL";
        let output = expand_env_vars(input).unwrap();
        assert_eq!(output, "bob uses /bin/bash");
    }

    #[test]
    fn test_missing_var_unix() {
        unsafe {
            std::env::remove_var("DOES_NOT_EXIST");
        }
        let input = "This is $DOES_NOT_EXIST";
        let output = expand_env_vars(input).unwrap();
        assert_eq!(output, "This is ");
    }

    #[cfg(windows)]
    #[test]
    fn test_single_var_windows() {
        unsafe {
            std::env::set_var("USERNAME", "charlie");
        }
        let input = "User: %USERNAME%";
        let output = expand_env_vars(input).unwrap();
        assert_eq!(output, "User: charlie");
    }

    #[cfg(windows)]
    #[test]
    fn test_multiple_vars_windows() {
        unsafe {
            std::env::set_var("USERNAME", "charlie");
            std::env::set_var("APPDATA", "C:\\Users\\charlie\\AppData");
        }
        let input = "%USERNAME%'s config: %APPDATA%";
        let output = expand_env_vars(input).unwrap();
        assert_eq!(output, "charlie's config: C:\\Users\\charlie\\AppData");
    }

    #[cfg(windows)]
    #[test]
    fn test_missing_var_windows() {
        unsafe {
            std::env::remove_var("DOES_NOT_EXIST");
        }
        let input = "Value: %DOES_NOT_EXIST%";
        let output = expand_env_vars(input).unwrap();
        assert_eq!(output, "Value: ");
    }
}
