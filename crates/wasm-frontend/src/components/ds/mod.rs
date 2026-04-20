//! Design system section components.
//!
//! Each submodule renders one section of the design system reference page.
//! Shared helpers live at the module level; individual sections are submodules.

use html::content::Section;
use html::text_content::Division;

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

/// Normalize HTML for snapshot comparison.
///
/// Strips HTML comments, collapses whitespace, trims whitespace around tag
/// boundaries, and sorts attributes alphabetically so that attribute-order
/// differences between hand-written HTML and `html`-crate output don't cause
/// false failures.
#[cfg(test)]
pub(super) fn normalize_html(s: &str) -> String {
    // Strip HTML comments
    let no_comments: String = s
        .lines()
        .map(|l| {
            if let Some(start) = l.find("<!--") {
                if let Some(end) = l.find("-->") {
                    let mut s = l.to_string();
                    s.replace_range(start..end + 3, "");
                    return s;
                }
            }
            l.to_string()
        })
        .collect::<Vec<_>>()
        .join(" ");
    // Collapse whitespace
    let collapsed: String = no_comments.split_whitespace().collect::<Vec<_>>().join(" ");
    // Normalize tag boundaries: strip whitespace between tags and text
    let collapsed = collapsed
        .replace("><", "> <")
        .replace("> ", ">")
        .replace(" <", "<");
    let collapsed = collapsed.replace("><", "> <");
    // Sort attributes within each tag
    sort_html_attrs(&collapsed)
}

/// Re-order attributes inside `<tag ...>` alphabetically.
#[cfg(test)]
fn sort_html_attrs(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(open) = rest.find('<') {
        result.push_str(&rest[..open]);
        rest = &rest[open..];
        if let Some(close) = rest.find('>') {
            let tag = &rest[..=close];
            result.push_str(&sort_tag_attrs(tag));
            rest = &rest[close + 1..];
        } else {
            result.push_str(rest);
            break;
        }
    }
    result.push_str(rest);
    result
}

/// Sort attributes in a single HTML tag.
#[cfg(test)]
fn sort_tag_attrs(tag: &str) -> String {
    if tag.starts_with("</") || !tag.contains(' ') {
        return tag.to_string();
    }
    let inner = tag
        .trim_start_matches('<')
        .trim_end_matches('>')
        .trim_end_matches('/');
    let first_space = match inner.find(' ') {
        Some(i) => i,
        None => return tag.to_string(),
    };
    let tag_name = &inner[..first_space];
    let attr_str = inner[first_space..].trim();
    if attr_str.is_empty() {
        return tag.to_string();
    }

    let mut attrs = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = '"';
    for ch in attr_str.chars() {
        if in_quote {
            current.push(ch);
            if ch == quote_char {
                in_quote = false;
            }
        } else if ch == '"' || ch == '\'' {
            in_quote = true;
            quote_char = ch;
            current.push(ch);
        } else if ch == ' ' && !current.is_empty() {
            attrs.push(current.clone());
            current.clear();
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        attrs.push(current);
    }

    attrs.sort();
    let self_closing = tag.ends_with("/>");
    let suffix = if self_closing { " />" } else { ">" };
    format!("<{} {}{}", tag_name, attrs.join(" "), suffix)
}
