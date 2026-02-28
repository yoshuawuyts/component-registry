//! Pure logic extracted from the `Manager` and `Store` implementations.
//!
//! These functions contain no IO and can be unit-tested in isolation.

use oci_client::manifest::OciDescriptor;
use std::collections::HashSet;

/// Truncated digest length used in vendor filenames.
const DIGEST_PREFIX_LEN: usize = 12;

/// Compute the vendor filename for a cached layer.
///
/// The filename encodes the registry, repository, optional tag, and a
/// truncated image digest so that vendored files are human-identifiable.
#[must_use]
pub fn vendor_filename(
    registry: &str,
    repository: &str,
    tag: Option<&str>,
    digest: &str,
) -> String {
    let registry_part = registry.replace('.', "-");
    let repo_part = repository.replace('/', "-");
    let tag_part = tag.map(|t| format!("-{t}")).unwrap_or_default();
    let sha_part = digest.strip_prefix("sha256:").unwrap_or(digest);
    let short_sha = sha_part.get(..DIGEST_PREFIX_LEN).unwrap_or(sha_part);
    format!("{registry_part}-{repo_part}{tag_part}-{short_sha}.wasm")
}

/// Determine whether a sync from the meta-registry should proceed.
///
/// Returns `true` when enough time has elapsed since `last_synced_epoch`,
/// or when the last-sync timestamp is unknown.
#[must_use]
pub fn should_sync(last_synced_epoch: Option<i64>, sync_interval: u64, now_epoch: i64) -> bool {
    match last_synced_epoch {
        Some(last) => now_epoch - last >= sync_interval as i64,
        None => true,
    }
}

/// Filter manifest layers to only those with `application/wasm` media type.
#[must_use]
pub fn filter_wasm_layers(layers: &[OciDescriptor]) -> Vec<&OciDescriptor> {
    layers
        .iter()
        .filter(|l| l.media_type == "application/wasm")
        .collect()
}

/// Compute which layer digests are orphaned after removing a set of manifests.
///
/// Given the digests belonging to the manifests being deleted and the digests
/// belonging to all other (retained) manifests, returns those that appear only
/// in the deleted set and can safely be purged from the content store.
#[must_use]
pub fn compute_orphaned_layers(
    deleted_digests: &HashSet<String>,
    retained_digests: &HashSet<String>,
) -> Vec<String> {
    deleted_digests
        .difference(retained_digests)
        .cloned()
        .collect()
}

/// Classify a single tag as release, signature, or attestation.
///
/// OCI cosign conventions use `sha256-<hex>` prefixed tags:
///   - `.sig` suffix → signature tag
///   - `.att` suffix → attestation tag
///   - everything else → release tag
#[must_use]
pub fn classify_tag(tag: &str) -> TagKind {
    if tag.starts_with("sha256-") {
        if tag.ends_with(".sig") {
            TagKind::Signature
        } else if tag.ends_with(".att") {
            TagKind::Attestation
        } else {
            TagKind::Release
        }
    } else {
        TagKind::Release
    }
}

/// The kind of an OCI tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagKind {
    /// A normal release tag (e.g., `v1.0`, `latest`).
    Release,
    /// A cosign signature tag (e.g., `sha256-abc123.sig`).
    Signature,
    /// A cosign attestation tag (e.g., `sha256-abc123.att`).
    Attestation,
}

/// Sanitize a string into a valid WIT identifier.
///
/// WIT identifiers must match `[a-z][a-z0-9]*(-[a-z][a-z0-9]*)*`.
/// Returns `None` if the input cannot be sanitized into a valid identifier
/// (e.g. it contains only digits or special characters).
#[must_use]
pub fn sanitize_to_wit_identifier(input: &str) -> Option<String> {
    // Lowercase and replace non-alphanumeric characters with hyphens.
    let sanitized: String = input
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();

    // Collapse consecutive hyphens, strip leading/trailing hyphens.
    let collapsed: String = sanitized
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    // Strip leading digits (WIT identifiers must start with [a-z]).
    let trimmed = collapsed.trim_start_matches(|c: char| c.is_ascii_digit());

    // Strip a possible leading hyphen left after digit removal.
    let trimmed = trimmed.strip_prefix('-').unwrap_or(trimmed);

    if trimmed.is_empty() {
        return None;
    }

    Some(trimmed.to_string())
}

/// Derive a human-friendly component name for use in `wasm.toml`.
///
/// Follows this priority chain:
/// 1. **WIT package name** — strip the `@version` suffix.
/// 2. **OCI `image.title`** — sanitized to a WIT-legal identifier.
/// 3. **Last segment of the repository path** — sanitized; used when no
///    collision with `existing_names`.
/// 4. **Full repository path** — with `/` replaced by `-`; used on collision.
#[must_use]
pub fn derive_component_name(
    package_name: Option<&str>,
    oci_title: Option<&str>,
    repository: &str,
    existing_names: &HashSet<String>,
) -> String {
    // 1. WIT package name (strip @version).
    if let Some(name) = package_name {
        return name.split('@').next().unwrap_or(name).to_string();
    }

    // 2. OCI image.title annotation.
    if let Some(title) = oci_title
        && let Some(sanitized) = sanitize_to_wit_identifier(title)
    {
        return sanitized;
    }

    // 3. Last segment of the repository path.
    let last_segment = repository.rsplit('/').next().unwrap_or(repository);
    if let Some(sanitized) = sanitize_to_wit_identifier(last_segment)
        && !existing_names.contains(&sanitized)
    {
        return sanitized;
    }

    // 4. Full repository path (on collision or if last segment fails).
    sanitize_to_wit_identifier(&repository.replace('/', "-"))
        .unwrap_or_else(|| repository.to_string())
}

/// Classify a list of tags into `(release, signature, attestation)` buckets.
///
/// This is a convenience wrapper around [`classify_tag`] that partitions
/// a slice of tags into three vectors.
#[must_use]
pub fn classify_tags(tags: &[String]) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut release = Vec::new();
    let mut signature = Vec::new();
    let mut attestation = Vec::new();

    for tag in tags {
        match classify_tag(tag) {
            TagKind::Release => release.push(tag.clone()),
            TagKind::Signature => signature.push(tag.clone()),
            TagKind::Attestation => attestation.push(tag.clone()),
        }
    }

    (release, signature, attestation)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── vendor_filename ─────────────────────────────────────────────────

    #[test]
    fn vendor_filename_basic() {
        let name = vendor_filename(
            "ghcr.io",
            "user/repo",
            Some("v1.0"),
            "sha256:abcdef1234567890",
        );
        assert_eq!(name, "ghcr-io-user-repo-v1.0-abcdef123456.wasm");
    }

    #[test]
    fn vendor_filename_no_tag() {
        let name = vendor_filename("ghcr.io", "user/repo", None, "sha256:abcdef1234567890");
        assert_eq!(name, "ghcr-io-user-repo-abcdef123456.wasm");
    }

    #[test]
    fn vendor_filename_short_digest() {
        let name = vendor_filename("ghcr.io", "user/repo", Some("latest"), "sha256:abc");
        assert_eq!(name, "ghcr-io-user-repo-latest-abc.wasm");
    }

    #[test]
    fn vendor_filename_no_sha256_prefix() {
        let name = vendor_filename("docker.io", "lib/hello", Some("1.0"), "rawdigest123456");
        assert_eq!(name, "docker-io-lib-hello-1.0-rawdigest123.wasm");
    }

    #[test]
    fn vendor_filename_nested_repository() {
        let name = vendor_filename(
            "ghcr.io",
            "org/team/component",
            Some("v2"),
            "sha256:0123456789abcdef",
        );
        assert_eq!(name, "ghcr-io-org-team-component-v2-0123456789ab.wasm");
    }

    #[test]
    fn vendor_filename_unknown_digest() {
        let name = vendor_filename("ghcr.io", "user/repo", None, "unknown");
        assert_eq!(name, "ghcr-io-user-repo-unknown.wasm");
    }

    // ── should_sync ─────────────────────────────────────────────────────

    #[test]
    fn should_sync_no_previous() {
        assert!(should_sync(None, 3600, 1000));
    }

    #[test]
    fn should_sync_stale() {
        assert!(should_sync(Some(1000), 3600, 5000));
    }

    #[test]
    fn should_sync_fresh() {
        assert!(!should_sync(Some(1000), 3600, 2000));
    }

    #[test]
    fn should_sync_exact_boundary() {
        // Exactly at the interval boundary should trigger sync.
        assert!(should_sync(Some(0), 3600, 3600));
    }

    // ── filter_wasm_layers ──────────────────────────────────────────────

    #[test]
    fn filter_wasm_layers_mixed() {
        let layers = vec![
            OciDescriptor {
                media_type: "application/wasm".to_string(),
                digest: "sha256:aaa".to_string(),
                size: 100,
                urls: None,
                annotations: None,
            },
            OciDescriptor {
                media_type: "application/vnd.oci.image.config.v1+json".to_string(),
                digest: "sha256:bbb".to_string(),
                size: 50,
                urls: None,
                annotations: None,
            },
            OciDescriptor {
                media_type: "application/wasm".to_string(),
                digest: "sha256:ccc".to_string(),
                size: 200,
                urls: None,
                annotations: None,
            },
        ];
        let wasm = filter_wasm_layers(&layers);
        assert_eq!(wasm.len(), 2);
        assert_eq!(wasm[0].digest, "sha256:aaa");
        assert_eq!(wasm[1].digest, "sha256:ccc");
    }

    #[test]
    fn filter_wasm_layers_none() {
        let layers = vec![OciDescriptor {
            media_type: "application/json".to_string(),
            digest: "sha256:xxx".to_string(),
            size: 10,
            urls: None,
            annotations: None,
        }];
        assert!(filter_wasm_layers(&layers).is_empty());
    }

    #[test]
    fn filter_wasm_layers_empty() {
        assert!(filter_wasm_layers(&[]).is_empty());
    }

    // ── compute_orphaned_layers ─────────────────────────────────────────

    #[test]
    fn orphaned_layers_disjoint() {
        let deleted: HashSet<String> = ["sha256:aaa", "sha256:bbb"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let retained: HashSet<String> = ["sha256:ccc"].iter().map(|s| s.to_string()).collect();
        let mut orphaned = compute_orphaned_layers(&deleted, &retained);
        orphaned.sort();
        assert_eq!(orphaned, vec!["sha256:aaa", "sha256:bbb"]);
    }

    #[test]
    fn orphaned_layers_overlap() {
        let deleted: HashSet<String> = ["sha256:aaa", "sha256:shared"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let retained: HashSet<String> = ["sha256:shared", "sha256:ccc"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let orphaned = compute_orphaned_layers(&deleted, &retained);
        assert_eq!(orphaned, vec!["sha256:aaa"]);
    }

    #[test]
    fn orphaned_layers_all_shared() {
        let deleted: HashSet<String> = ["sha256:aaa"].iter().map(|s| s.to_string()).collect();
        let retained: HashSet<String> = ["sha256:aaa"].iter().map(|s| s.to_string()).collect();
        let orphaned = compute_orphaned_layers(&deleted, &retained);
        assert!(orphaned.is_empty());
    }

    #[test]
    fn orphaned_layers_empty_retained() {
        let deleted: HashSet<String> = ["sha256:aaa", "sha256:bbb"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let retained: HashSet<String> = HashSet::new();
        let mut orphaned = compute_orphaned_layers(&deleted, &retained);
        orphaned.sort();
        assert_eq!(orphaned, vec!["sha256:aaa", "sha256:bbb"]);
    }

    #[test]
    fn orphaned_layers_empty_deleted() {
        let deleted: HashSet<String> = HashSet::new();
        let retained: HashSet<String> = ["sha256:aaa"].iter().map(|s| s.to_string()).collect();
        let orphaned = compute_orphaned_layers(&deleted, &retained);
        assert!(orphaned.is_empty());
    }

    // ── classify_tag / classify_tags ────────────────────────────────────

    #[test]
    fn classify_tag_release() {
        assert_eq!(classify_tag("v1.0"), TagKind::Release);
        assert_eq!(classify_tag("latest"), TagKind::Release);
    }

    #[test]
    fn classify_tag_signature() {
        assert_eq!(classify_tag("sha256-abc123def456.sig"), TagKind::Signature);
    }

    #[test]
    fn classify_tag_attestation() {
        assert_eq!(
            classify_tag("sha256-abc123def456.att"),
            TagKind::Attestation
        );
    }

    #[test]
    fn classify_tag_sha256_without_suffix() {
        // sha256- prefix but no .sig or .att → release
        assert_eq!(classify_tag("sha256-abc123def456"), TagKind::Release);
    }

    #[test]
    fn classify_tags_mixed() {
        let tags: Vec<String> = vec![
            "v1.0".into(),
            "latest".into(),
            "sha256-abc123.sig".into(),
            "sha256-abc123.att".into(),
            "sha256-def456".into(),
        ];
        let (release, signature, attestation) = classify_tags(&tags);
        assert_eq!(release, vec!["v1.0", "latest", "sha256-def456"]);
        assert_eq!(signature, vec!["sha256-abc123.sig"]);
        assert_eq!(attestation, vec!["sha256-abc123.att"]);
    }

    #[test]
    fn classify_tags_empty() {
        let (release, signature, attestation) = classify_tags(&[]);
        assert!(release.is_empty());
        assert!(signature.is_empty());
        assert!(attestation.is_empty());
    }

    #[test]
    fn classify_tags_all_release() {
        let tags: Vec<String> = vec!["v1.0".into(), "latest".into(), "stable".into()];
        let (release, signature, attestation) = classify_tags(&tags);
        assert_eq!(release.len(), 3);
        assert!(signature.is_empty());
        assert!(attestation.is_empty());
    }

    // ── sanitize_to_wit_identifier ──────────────────────────────────────

    #[test]
    fn sanitize_already_valid() {
        assert_eq!(
            sanitize_to_wit_identifier("fetch"),
            Some("fetch".to_string())
        );
    }

    #[test]
    fn sanitize_uppercase() {
        assert_eq!(
            sanitize_to_wit_identifier("Fetch"),
            Some("fetch".to_string())
        );
    }

    #[test]
    fn sanitize_underscores() {
        assert_eq!(
            sanitize_to_wit_identifier("my_component"),
            Some("my-component".to_string())
        );
    }

    #[test]
    fn sanitize_leading_digits() {
        assert_eq!(
            sanitize_to_wit_identifier("123fetch"),
            Some("fetch".to_string())
        );
    }

    #[test]
    fn sanitize_all_digits() {
        assert_eq!(sanitize_to_wit_identifier("12345"), None);
    }

    #[test]
    fn sanitize_empty_after_sanitization() {
        assert_eq!(sanitize_to_wit_identifier("!!!"), None);
    }

    #[test]
    fn sanitize_complex() {
        assert_eq!(
            sanitize_to_wit_identifier("My Cool_Fetch.Component"),
            Some("my-cool-fetch-component".to_string())
        );
    }

    // ── derive_component_name ───────────────────────────────────────────

    #[test]
    fn derive_name_wit_package_name() {
        let existing = HashSet::new();
        let name = derive_component_name(
            Some("wasi:http@0.2.10"),
            None,
            "webassembly/wasi-http",
            &existing,
        );
        assert_eq!(name, "wasi:http");
    }

    #[test]
    fn derive_name_oci_title() {
        let existing = HashSet::new();
        let name = derive_component_name(
            None,
            Some("My Fetch Component"),
            "yoshuawuyts/fetch",
            &existing,
        );
        assert_eq!(name, "my-fetch-component");
    }

    #[test]
    fn derive_name_last_segment() {
        let existing = HashSet::new();
        let name = derive_component_name(None, None, "yoshuawuyts/fetch", &existing);
        assert_eq!(name, "fetch");
    }

    #[test]
    fn derive_name_collision() {
        let mut existing = HashSet::new();
        existing.insert("fetch".to_string());
        let name = derive_component_name(None, None, "yoshuawuyts/fetch", &existing);
        assert_eq!(name, "yoshuawuyts-fetch");
    }

    #[test]
    fn derive_name_repo_with_underscores_dots() {
        let existing = HashSet::new();
        let name = derive_component_name(None, None, "my_org/my.component", &existing);
        assert_eq!(name, "my-component");
    }

    #[test]
    fn derive_name_repo_with_underscores_dots_collision() {
        let mut existing = HashSet::new();
        existing.insert("my-component".to_string());
        let name = derive_component_name(None, None, "my_org/my.component", &existing);
        assert_eq!(name, "my-org-my-component");
    }

    #[test]
    fn derive_name_oci_title_invalid_chars() {
        let existing = HashSet::new();
        let name = derive_component_name(None, Some("!!!"), "yoshuawuyts/fetch", &existing);
        // Title sanitizes to empty → falls through to last segment
        assert_eq!(name, "fetch");
    }

    #[test]
    fn derive_name_oci_title_sanitizes_to_empty() {
        let existing = HashSet::new();
        let name = derive_component_name(None, Some("12345"), "yoshuawuyts/fetch", &existing);
        // Title is all digits → sanitizes to None → falls through
        assert_eq!(name, "fetch");
    }
}
