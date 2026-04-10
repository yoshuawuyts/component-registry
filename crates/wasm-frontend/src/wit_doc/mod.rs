//! Parse WIT text into a rich document model using `wit-parser` directly.
//!
//! Converts WIT source text into a [`WitDocument`] that captures every
//! interface, type, function, world, and doc comment — with pre-resolved
//! URLs for cross-linking.

mod convert;
pub(crate) mod types;

pub(crate) use types::*;

use std::collections::HashMap;
use std::hash::BuildHasher;

/// Parse WIT source text into a [`WitDocument`].
///
/// # Arguments
///
/// * `wit_text` — WIT source (text form, not binary).
/// * `url_base` — base URL path for this package (e.g.
///   `"/wasi/http/0.2.11"`). All generated URLs are rooted here.
/// * `dep_urls` — maps dependency package names (e.g. `"wasi:io"`) to their
///   URL base (e.g. `"/wasi/io/0.2.2"`), enabling cross-package links.
///
/// # Errors
///
/// Returns an error if the WIT text fails to parse.
pub(crate) fn parse_wit_doc<S: BuildHasher>(
    wit_text: &str,
    url_base: &str,
    dep_urls: &HashMap<String, String, S>,
) -> anyhow::Result<WitDocument> {
    let standard: HashMap<String, String> = dep_urls
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    convert::convert(wit_text, url_base, &standard)
}
