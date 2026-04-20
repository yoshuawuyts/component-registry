//! 20 — Empty State.

use html::text_content::Division;

const SVG_CHAT: &str = r#"<svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" /></svg>"#;
const SVG_PLUS: &str = r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14" /><path d="M12 5v14" /></svg>"#;

/// Render this section.
pub(crate) fn render() -> String {
    let content = Division::builder()
        .class("border border-line rounded-lg p-12 text-center bg-surface")
        .division(|icon| {
            icon.class("mx-auto h-12 w-12 grid place-items-center rounded-full bg-surfaceMuted text-ink-500")
                .text(SVG_CHAT)
        })
        .division(|d| d.class("mt-4 text-[16px] font-semibold tracking-tight").text("No lorem yet"))
        .paragraph(|p| {
            p.class("mt-1 text-[13px] text-ink-500 max-w-xs mx-auto")
                .text("Pellentesque habitant morbi tristique. Get started by creating your first entry.")
        })
        .button(|btn| {
            btn.class("mt-5 h-9 px-3 inline-flex items-center gap-2 rounded-lg bg-surfaceMuted text-ink-900 text-[13px] hover:bg-ink-300")
                .text(SVG_PLUS)
                .text(" Create entry")
        })
        .build()
        .to_string();

    super::section(
        "empty",
        "20",
        "Empty State",
        "Centered illustration glyph, title, body, and primary CTA. Used for empty tables, search misses, and first-run views.",
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
