//! XDG Base Directory helpers.
//!
//! These helpers follow the [XDG Base Directory Specification] directly,
//! rather than using the platform-specific mappings provided by the `dirs`
//! crate. This ensures the configuration directory is always
//! `$XDG_CONFIG_HOME` (defaulting to `~/.config`) on every OS.
//!
//! [XDG Base Directory Specification]: https://specifications.freedesktop.org/basedir-spec/latest/

use std::env;
use std::path::PathBuf;

/// Return the XDG config home directory.
///
/// Uses `$XDG_CONFIG_HOME` if set, otherwise falls back to `$HOME/.config`.
/// If neither environment variable is available, returns `.config` as a
/// relative path (resolved against the current working directory by callers).
pub(crate) fn xdg_config_home() -> PathBuf {
    if let Some(val) = env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(val);
    }
    dirs::home_dir().map_or_else(|| PathBuf::from(".config"), |h| h.join(".config"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xdg_config_home_returns_non_empty_path() {
        let path = xdg_config_home();
        // The path must be non-empty regardless of the environment.
        assert!(!path.as_os_str().is_empty());
    }

    #[test]
    fn xdg_config_home_defaults_to_dot_config_when_env_unset() {
        let path = xdg_config_home();
        // When $XDG_CONFIG_HOME is not set the path must end with ".config".
        // When it *is* set we simply accept whatever the env provides.
        if env::var_os("XDG_CONFIG_HOME").is_none() {
            assert!(
                path.ends_with(".config"),
                "expected path to end with .config, got: {}",
                path.display()
            );
        }
    }
}
