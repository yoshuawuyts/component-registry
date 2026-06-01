//! `component registry publish` subcommand.
//!
//! Opens a prefilled "Registry entry" issue on the registry's GitHub
//! repository so a maintainer's automation can turn it into a pull request.
//! This does not touch the local store or the network directly; it only
//! constructs a URL (and optionally launches the system browser).

#![allow(clippy::print_stdout, clippy::print_stderr)]

use anyhow::{Result, bail};
use std::fmt;
use std::fmt::Write as _;

/// Default GitHub repository hosting the component registry.
///
/// This mirrors the manager's compile-time default registry: the registry's
/// issue forms live in this repository.
const DEFAULT_REGISTRY_REPO: &str = "yoshuawuyts/component-registry";

/// File name of the registry-entry issue form template.
const ISSUE_TEMPLATE: &str = "registry-entry.yml";

/// Whether a registry entry describes a component or a WIT interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub(crate) enum Kind {
    /// A Wasm Component.
    Component,
    /// A WIT interface.
    Interface,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Kind::Component => "component",
            Kind::Interface => "interface",
        };
        f.write_str(s)
    }
}

/// Open a prefilled registry-entry issue to add a component or interface.
///
/// The package is given as a WIT-style `namespace:package` name; combined with
/// `--kind` and `--repository` this prefills the registry's issue form so the
/// entry can be reviewed and merged.
#[derive(clap::Args)]
pub(crate) struct PublishOpts {
    /// The package to add, as a WIT-style name (e.g., `wasi:http`).
    ///
    /// A trailing `@version` is not accepted; the registry tracks packages,
    /// not individual versions.
    name: String,

    /// Whether the entry is a component or a WIT interface.
    #[arg(long, value_enum)]
    kind: Kind,

    /// Path within the OCI registry where the package is published
    /// (e.g., `components/wordmark`).
    #[arg(long)]
    repository: String,

    /// OCI registry base for a brand-new namespace (e.g., `ghcr.io/my-org`).
    ///
    /// Only required when the namespace does not yet exist.
    #[arg(long)]
    registry: Option<String>,

    /// GitHub repository hosting the registry, as `owner/name`.
    #[arg(long, default_value = DEFAULT_REGISTRY_REPO)]
    repo: String,

    /// Print the issue URL without launching a browser.
    #[arg(long)]
    no_open: bool,
}

impl PublishOpts {
    pub(crate) fn run(self) -> Result<()> {
        let (namespace, package) = parse_wit_name(&self.name)?;
        validate_repo(&self.repo)?;

        let url = build_issue_url(
            &self.repo,
            self.kind,
            namespace,
            package,
            &self.repository,
            self.registry.as_deref(),
        );

        println!("{url}");

        if !self.no_open
            && let Err(err) = open_in_browser(&url)
        {
            eprintln!("Could not open a browser ({err}); open the URL above manually.");
        }

        Ok(())
    }
}

/// Split a WIT-style `namespace:package` name into its two parts.
///
/// Rejects versions (`@`), path separators (`/`), and anything that is not
/// exactly one non-empty namespace and one non-empty package.
fn parse_wit_name(name: &str) -> Result<(&str, &str)> {
    let Some((namespace, package)) = name.split_once(':') else {
        bail!("'{name}' is not a WIT-style name; expected `namespace:package` (e.g., `wasi:http`)");
    };

    if namespace.is_empty() || package.is_empty() {
        bail!("'{name}' must have a non-empty namespace and package (e.g., `wasi:http`)");
    }
    if package.contains(':') {
        bail!("'{name}' has too many ':' separators; expected `namespace:package`");
    }
    if name.contains('@') {
        bail!("'{name}' must not include a version; the registry tracks packages, not versions");
    }
    if namespace.contains('/') || package.contains('/') {
        bail!("'{name}' must not contain '/'; use --repository for the OCI path");
    }

    Ok((namespace, package))
}

/// Validate a `owner/name` GitHub repository slug used in the URL path.
fn validate_repo(repo: &str) -> Result<()> {
    let Some((owner, name)) = repo.split_once('/') else {
        bail!(
            "--repo '{repo}' must be in `owner/name` form (e.g., `yoshuawuyts/component-registry`)"
        );
    };
    if owner.is_empty() || name.is_empty() || name.contains('/') {
        bail!(
            "--repo '{repo}' must be in `owner/name` form (e.g., `yoshuawuyts/component-registry`)"
        );
    }
    if repo.contains(['?', '#', ' ']) {
        bail!("--repo '{repo}' contains characters that are not valid in a repository name");
    }
    Ok(())
}

/// Build the prefilled GitHub issue-form URL.
///
/// Each query key matches a field `id` in `registry-entry.yml`, so GitHub
/// populates the corresponding form fields. Query values are percent-encoded.
#[must_use]
fn build_issue_url(
    repo: &str,
    kind: Kind,
    namespace: &str,
    package: &str,
    repository: &str,
    registry: Option<&str>,
) -> String {
    let mut url = format!("https://github.com/{repo}/issues/new?template={ISSUE_TEMPLATE}");
    write!(url, "&kind={}", encode(&kind.to_string())).expect("writing to a String cannot fail");
    write!(url, "&namespace={}", encode(namespace)).expect("writing to a String cannot fail");
    write!(url, "&package={}", encode(package)).expect("writing to a String cannot fail");
    write!(url, "&repository={}", encode(repository)).expect("writing to a String cannot fail");
    if let Some(registry) = registry.filter(|r| !r.is_empty()) {
        write!(url, "&registry={}", encode(registry)).expect("writing to a String cannot fail");
    }
    url
}

/// Percent-encode a query value, passing through only RFC 3986 unreserved
/// characters and encoding every other UTF-8 byte as `%XX`.
#[must_use]
fn encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for &byte in value.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            out.push(byte as char);
        } else {
            write!(out, "%{byte:02X}").expect("writing to a String cannot fail");
        }
    }
    out
}

/// Launch the system browser for `url` using a platform-native handler.
///
/// Uses `Command` directly (never a shell) so URL metacharacters such as `&`
/// and `%` cannot be reinterpreted.
fn open_in_browser(url: &str) -> Result<()> {
    use std::process::Command;

    #[cfg(target_os = "macos")]
    let mut command = {
        let mut c = Command::new("open");
        c.arg(url);
        c
    };

    #[cfg(target_os = "windows")]
    let mut command = {
        // Avoid `cmd /C start`, which reinterprets `&` and `%` in the URL.
        let mut c = Command::new("rundll32.exe");
        c.args(["url.dll,FileProtocolHandler", url]);
        c
    };

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let mut command = {
        let mut c = Command::new("xdg-open");
        c.arg(url);
        c
    };

    let status = command.status()?;
    if !status.success() {
        bail!("browser launcher exited with status {status}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_wit_name() {
        assert_eq!(parse_wit_name("wasi:http").unwrap(), ("wasi", "http"));
        assert_eq!(parse_wit_name("yosh:fetch").unwrap(), ("yosh", "fetch"));
    }

    #[test]
    fn rejects_invalid_wit_names() {
        assert!(parse_wit_name("nocolon").is_err());
        assert!(parse_wit_name(":http").is_err());
        assert!(parse_wit_name("wasi:").is_err());
        assert!(parse_wit_name("wasi:http@0.2.0").is_err());
        assert!(parse_wit_name("wasi:http:extra").is_err());
        assert!(parse_wit_name("wasi:comp/onents").is_err());
    }

    #[test]
    fn validates_repo() {
        assert!(validate_repo("yoshuawuyts/component-registry").is_ok());
        assert!(validate_repo("noslash").is_err());
        assert!(validate_repo("owner/").is_err());
        assert!(validate_repo("/name").is_err());
        assert!(validate_repo("a/b/c").is_err());
        assert!(validate_repo("owner/na me").is_err());
    }

    #[test]
    fn encodes_query_values() {
        assert_eq!(encode("components/wordmark"), "components%2Fwordmark");
        assert_eq!(encode("ghcr.io/my-org"), "ghcr.io%2Fmy-org");
        assert_eq!(encode("ghcr.io:5000/org"), "ghcr.io%3A5000%2Forg");
        assert_eq!(encode("plain-name_1.0"), "plain-name_1.0");
    }

    #[test]
    fn builds_url_without_registry() {
        let url = build_issue_url(
            "yoshuawuyts/component-registry",
            Kind::Component,
            "yosh",
            "fetch",
            "components/fetch",
            None,
        );
        assert_eq!(
            url,
            "https://github.com/yoshuawuyts/component-registry/issues/new\
             ?template=registry-entry.yml&kind=component&namespace=yosh\
             &package=fetch&repository=components%2Ffetch"
        );
    }

    #[test]
    fn builds_url_with_registry_for_new_namespace() {
        let url = build_issue_url(
            "yoshuawuyts/component-registry",
            Kind::Interface,
            "acme",
            "types",
            "iface/types",
            Some("ghcr.io/acme"),
        );
        assert!(url.contains("&kind=interface"));
        assert!(url.contains("&namespace=acme"));
        assert!(url.contains("&package=types"));
        assert!(url.contains("&repository=iface%2Ftypes"));
        assert!(url.contains("&registry=ghcr.io%2Facme"));
    }

    #[test]
    fn omits_empty_registry() {
        let url = build_issue_url(
            "owner/repo",
            Kind::Component,
            "ns",
            "pkg",
            "ns/pkg",
            Some(""),
        );
        assert!(!url.contains("registry="));
    }
}
