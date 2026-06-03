//! Escaping helpers for safely embedding registry-controlled (untrusted)
//! data into raw HTML and inline JavaScript.
//!
//! The `html` crate performs no automatic escaping: both element text and
//! attribute values are rendered verbatim, and [`crate::markdown`] embeds
//! rendered HTML directly. Any value derived from registry data — OCI
//! annotations, package metadata, user search queries — MUST therefore be
//! escaped before it is interpolated into raw HTML or a `<script>` body.

/// Escape a string for safe inclusion in HTML text content.
///
/// Escapes the five characters that are significant in HTML text and
/// double/single-quoted attribute contexts: `&`, `<`, `>`, `"`, and `'`.
#[must_use]
pub(crate) fn escape_html_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Escape a string for safe inclusion in a quoted HTML attribute value.
///
/// Uses the same entity set as [`escape_html_text`]; escaping both `"` and
/// `'` makes the result safe inside either double- or single-quoted
/// attribute values.
#[must_use]
pub(crate) fn escape_html_attr(input: &str) -> String {
    escape_html_text(input)
}

/// Escape a string for safe inclusion inside a quoted JavaScript string
/// literal embedded in an inline `<script>` element.
///
/// Backslash and quotes are backslash-escaped; `<` and `>` are emitted as
/// `\x3C`/`\x3E` so the sequence `</script>` cannot terminate the script
/// element early; line terminators (including U+2028/U+2029) are escaped.
/// The browser decodes these escapes, so the runtime string value is
/// preserved.
#[must_use]
pub(crate) fn escape_js_string(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '"' => out.push_str("\\\""),
            '<' => out.push_str("\\x3C"),
            '>' => out.push_str("\\x3E"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            _ => out.push(ch),
        }
    }
    out
}

/// Sanitize a URL for use in an `href`/`src` attribute, neutralizing
/// dangerous schemes such as `javascript:`, `data:`, and `vbscript:`.
///
/// Relative URLs (path, query, fragment, or protocol-relative) and the
/// `http`, `https`, and `mailto` schemes are allowed. Anything else is
/// replaced with `"#"`. The returned value is NOT attribute-escaped;
/// callers must still pass it through [`escape_html_attr`].
#[must_use]
pub(crate) fn sanitize_url(url: &str) -> String {
    if is_safe_url(url) {
        url.to_owned()
    } else {
        "#".to_owned()
    }
}

/// Return whether `url` is safe to use as a link target.
fn is_safe_url(url: &str) -> bool {
    let trimmed = url.trim();
    match trimmed.find(':') {
        // No scheme separator: a relative URL.
        None => true,
        Some(idx) => {
            let before = &trimmed[..idx];
            // If a path/query/fragment separator appears before the first
            // ':', the ':' is part of the path and there is no scheme.
            if before.contains('/') || before.contains('?') || before.contains('#') {
                return true;
            }
            matches!(
                before.to_ascii_lowercase().as_str(),
                "http" | "https" | "mailto"
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_html_text_metacharacters() {
        let injected = "</span><script>alert(1)</script>";
        let escaped = escape_html_text(injected);
        assert!(!escaped.contains("<script>"));
        assert!(!escaped.contains("</span>"));
        assert_eq!(
            escaped,
            "&lt;/span&gt;&lt;script&gt;alert(1)&lt;/script&gt;"
        );
    }

    #[test]
    fn escapes_attribute_breakout() {
        let injected = r#""><img src=x onerror=alert(1)>"#;
        let escaped = escape_html_attr(injected);
        assert!(!escaped.contains("\"><img"));
        assert!(escaped.starts_with("&quot;&gt;"));
    }

    #[test]
    fn leaves_safe_text_unchanged() {
        assert_eq!(escape_html_text("wasi:cli"), "wasi:cli");
        assert_eq!(escape_html_attr("/wasi/cli/0.2.0"), "/wasi/cli/0.2.0");
    }

    #[test]
    fn escapes_js_script_terminator_and_quotes() {
        let injected = "');</script><script>alert(1)//";
        let escaped = escape_js_string(injected);
        assert!(!escaped.contains("</script>"));
        assert!(escaped.contains("\\');"));
        assert!(escaped.contains("\\x3C"));
    }

    #[test]
    fn sanitizes_dangerous_url_schemes() {
        assert_eq!(sanitize_url("javascript:alert(1)"), "#");
        assert_eq!(sanitize_url("JavaScript:alert(1)"), "#");
        assert_eq!(sanitize_url("data:text/html,<script>"), "#");
        assert_eq!(sanitize_url("  javascript:alert(1)"), "#");
    }

    #[test]
    fn allows_safe_urls() {
        assert_eq!(
            sanitize_url("https://example.com/x"),
            "https://example.com/x"
        );
        assert_eq!(sanitize_url("/wasi/cli"), "/wasi/cli");
        assert_eq!(sanitize_url("#anchor"), "#anchor");
        assert_eq!(sanitize_url("mailto:a@b.com"), "mailto:a@b.com");
        assert_eq!(sanitize_url("./relative/path"), "./relative/path");
    }
}
