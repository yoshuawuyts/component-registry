//! 16 — Dropdown.

use html::text_content::Division;

const SVG_EDIT: &str = r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z" /><path d="m15 5 4 4" /></svg>"#;
const SVG_COPY: &str = r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" /><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" /></svg>"#;
const SVG_SHARE: &str = r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" /><polyline points="16 6 12 2 8 6" /><line x1="12" x2="12" y1="2" y2="15" /></svg>"#;
const SVG_DELETE: &str = r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18" /><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" /><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" /></svg>"#;

/// Render this section.
pub(crate) fn render() -> String {
    let content = Division::builder()
        .class("p-12 bg-canvas border border-line rounded-lg flex items-start justify-center")
        .division(|menu| {
            menu.class("w-56 rounded-md bg-surface border border-line shadow-tooltip py-1 text-[13px]")
                .division(|lbl| {
                    lbl.class("px-3 py-1.5 text-[11px] mono uppercase tracking-wider text-ink-400")
                        .text("Aenean")
                })
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2")
                        .text(SVG_EDIT)
                        .text(" Edit lorem")
                })
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2")
                        .text(SVG_COPY)
                        .text(" Duplicate")
                })
                .division(|sep| sep.class("my-1 border-t border-lineSoft"))
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2")
                        .text(SVG_SHARE)
                        .text(" Share")
                        .span(|s| s.class("ml-auto text-[11px] mono text-ink-400").text("\u{2318}S"))
                })
                .division(|sep| sep.class("my-1 border-t border-lineSoft"))
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2 text-negative")
                        .text(SVG_DELETE)
                        .text(" Delete")
                })
        })
        .build()
        .to_string();

    super::section(
        "dropdown",
        "16",
        "Dropdown",
        "Floating menu on white. 1px gray border + tooltip-grade shadow. Section dividers separate logical groups.",
        &content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(render());
    }
}
