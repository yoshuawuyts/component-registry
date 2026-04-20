//! Design system section components.
//!
//! Each submodule renders one section of the design system reference page.
//! Shared helpers live at the module level; individual sections are submodules.

use html::content::Section;

/// Render a standard design-system section with the two-column
/// `[200px | 1fr]` grid layout used by every numbered section.
///
/// `id` is the anchor, `num` the section number label (e.g. `"01"`),
/// `title` the heading text, `desc` the synopsis paragraph, and `content_fn`
/// builds the right-hand column contents.
pub(crate) fn section(id: &str, num: &str, title: &str, desc: &str, content: &str) -> String {
    let id = id.to_owned();
    let num = num.to_owned();
    let title = title.to_owned();
    let desc = desc.to_owned();
    let content = content.to_owned();
    let sec = Section::builder()
        .id(id)
        .class("pt-12 md:pt-16")
        .division(|grid| {
            grid.class("grid md:grid-cols-[200px_1fr] gap-6 md:gap-12")
                .division(|left| {
                    left.division(|n| {
                        n.class("text-[12px] mono uppercase tracking-wider text-ink-500")
                            .text(num.clone())
                    })
                    .heading_2(|h| {
                        h.class("mt-2 text-[24px] font-semibold tracking-tight")
                            .text(title.clone())
                    })
                    .paragraph(|p| {
                        p.class("mt-2 text-[13px] text-ink-500 leading-relaxed")
                            .text(desc.clone())
                    })
                })
                .text(content.clone())
        })
        .build();
    sec.to_string()
}

/// Render a subsection heading (h3).
/// Render a subsection heading (h3).
#[allow(dead_code)]
pub(crate) fn sub(text: &str) -> String {
    let text = text.to_owned();
    html::content::Heading3::builder()
        .class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
        .text(text)
        .build()
        .to_string()
}

pub(crate) mod c01_sidebar;
pub(crate) mod c02_toc;
pub(crate) mod c03_page_header;
pub(crate) mod c04_item_list;
pub(crate) mod c05_item_details;
pub(crate) mod c06_navbar;
pub(crate) mod header;
pub(crate) mod part_two;
pub(crate) mod s01_color;
pub(crate) mod s02_typography;
pub(crate) mod s03_spacing;
pub(crate) mod s04_elevation;
pub(crate) mod s05_buttons;
pub(crate) mod s06_tabs;
pub(crate) mod s07_navigation;
pub(crate) mod s08_code;
pub(crate) mod s09_labels;
pub(crate) mod s10_tooltip;
pub(crate) mod s11_table;
pub(crate) mod s12_icons;
pub(crate) mod s13_fields;
pub(crate) mod s14_toggles;
pub(crate) mod s15_badges;
pub(crate) mod s16_dropdown;
pub(crate) mod s17_modal;
pub(crate) mod s18_breadcrumb;
pub(crate) mod s19_progress;
pub(crate) mod s20_empty;
pub(crate) mod s21_grid;
pub(crate) mod s22_regions;
pub(crate) mod s23_motion;
pub(crate) mod s24_details;
pub(crate) mod toc;

/// Crude HTML pretty-printer for snapshot tests.
///
/// Inserts newlines before opening/closing tags and indents by nesting depth.
/// Not spec-compliant — just enough to make snapshot diffs readable.
#[cfg(test)]
pub(super) fn pretty_html(html: &str) -> String {
    let mut out = String::with_capacity(html.len() * 2);
    let mut depth: usize = 0;
    let mut i = 0;
    let bytes = html.as_bytes();

    while i < bytes.len() {
        if bytes[i] == b'<' {
            // Find the end of this tag.
            let tag_end = match memchr(b'>', bytes, i) {
                Some(pos) => pos,
                None => break,
            };
            let tag = &html[i..=tag_end];

            let is_close = tag.starts_with("</");
            let is_void = tag.ends_with("/>") || is_void_element(tag);

            if is_close {
                depth = depth.saturating_sub(1);
            }

            // Newline + indent before the tag.
            if !out.is_empty() {
                out.push('\n');
            }
            indent(&mut out, depth);
            out.push_str(tag);

            if !is_close && !is_void {
                depth += 1;
            }

            i = tag_end + 1;
        } else {
            // Text node — collect until next '<'.
            let text_end = memchr(b'<', bytes, i).unwrap_or(bytes.len());
            let text = &html[i..text_end];
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                out.push('\n');
                indent(&mut out, depth);
                out.push_str(trimmed);
            }
            i = text_end;
        }
    }
    out
}

#[cfg(test)]
fn memchr(needle: u8, haystack: &[u8], start: usize) -> Option<usize> {
    haystack[start..]
        .iter()
        .position(|&b| b == needle)
        .map(|p| p + start)
}

#[cfg(test)]
fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

#[cfg(test)]
fn is_void_element(tag: &str) -> bool {
    // Extract just the tag name.
    let name = tag
        .trim_start_matches('<')
        .split(|c: char| c.is_whitespace() || c == '/' || c == '>')
        .next()
        .unwrap_or("");
    matches!(
        name,
        "br" | "hr" | "img" | "input" | "meta" | "link" | "source" | "area" | "base" | "col"
    )
}
