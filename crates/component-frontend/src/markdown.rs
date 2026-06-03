//! Markdown-to-HTML rendering for documentation text.

/// Render a markdown string to HTML.
///
/// Uses `pulldown-cmark` to convert doc comments into HTML. Registry
/// documentation is untrusted, so raw HTML in the source is downgraded to
/// escaped text (instead of being passed through verbatim) and link/image
/// destinations with dangerous URL schemes (e.g. `javascript:`) are
/// neutralized. See [`crate::escape::sanitize_url`].
#[must_use]
pub(crate) fn render(input: &str) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(input, options).map(sanitize_event);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

/// Neutralize unsafe events emitted by the markdown parser.
///
/// Raw HTML is turned into text (so `push_html` escapes it), and link/image
/// destinations are passed through [`crate::escape::sanitize_url`].
fn sanitize_event(event: pulldown_cmark::Event<'_>) -> pulldown_cmark::Event<'_> {
    use pulldown_cmark::{Event, Tag};

    match event {
        Event::Html(html) | Event::InlineHtml(html) => Event::Text(html),
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => Event::Start(Tag::Link {
            link_type,
            dest_url: crate::escape::sanitize_url(&dest_url).into(),
            title,
            id,
        }),
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => Event::Start(Tag::Image {
            link_type,
            dest_url: crate::escape::sanitize_url(&dest_url).into(),
            title,
            id,
        }),
        other => other,
    }
}

/// Standard CSS class for rendered doc comment blocks.
pub(crate) const DOC_CLASS: &str = "text-base text-ink-500 leading-relaxed prose-doc";

/// Render markdown and wrap in a styled `<div>`.
///
/// Applies prose-like styling classes for rendered documentation.
#[must_use]
pub(crate) fn render_block(input: &str, class: &str) -> String {
    let html = render(input);
    format!(r#"<div class="{class}">{html}</div>"#)
}

/// Render a short markdown string as inline HTML.
///
/// Strips the outer `<p>` wrapper that pulldown-cmark adds for single
/// paragraphs, making the result safe for use inside table cells,
/// list items, and other inline contexts.
#[must_use]
pub(crate) fn render_inline(input: &str) -> String {
    let html = render(input);
    // Extract content from the first <p>...</p> only, stripping any
    // subsequent paragraphs that would break inline contexts.
    if let Some(start) = html.find("<p>") {
        let content_start = start + 3;
        if let Some(end) = html[content_start..].find("</p>") {
            return html[content_start..content_start + end].to_owned();
        }
    }
    // Fallback: return as-is if no <p> found
    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_raw_html_in_markdown() {
        let out = render("hello <script>alert(1)</script> world");
        assert!(!out.contains("<script>"));
        assert!(out.contains("&lt;script&gt;"));
    }

    #[test]
    fn neutralizes_javascript_links() {
        let out = render("[click](javascript:alert(1))");
        assert!(!out.contains("javascript:"));
        assert!(out.contains(r##"href="#""##));
    }

    #[test]
    fn preserves_safe_markdown() {
        let out = render("**bold** and [link](https://example.com)");
        assert!(out.contains("<strong>bold</strong>"));
        assert!(out.contains(r#"href="https://example.com""#));
    }
}
