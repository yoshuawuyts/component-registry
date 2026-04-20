//! 05 — Buttons.

use html::text_content::Division;

const SVG_CALENDAR: &str = r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="4" rx="2" /><path d="M16 2v4" /><path d="M8 2v4" /><path d="M3 10h18" /></svg>"#;
const SVG_CHEV: &str = r#"<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6" /></svg>"#;
const SVG_FILTER: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18" /><path d="M7 12h10" /><path d="M10 18h4" /></svg>"#;
const SVG_SAVE: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z" /><path d="M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7" /><path d="M7 3v4a1 1 0 0 0 1 1h7" /></svg>"#;
const SVG_UPLOAD: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3v12" /><path d="m17 8-5-5-5 5" /><path d="M21 21H3" /></svg>"#;

const ICON_BTN: &str =
    "h-8 w-8 grid place-items-center rounded-md hover:bg-surfaceMuted text-ink-700";

/// Render this section.
pub(crate) fn render() -> String {
    let content = Division::builder()
        .class("space-y-8")
        // Filled
        .division(|d| {
            d.heading_3(|h| h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3").text("Filled"))
                .division(|g| {
                    g.class("flex flex-wrap items-center gap-3")
                        .button(|b| {
                            b.class("h-8 px-3 inline-flex items-center gap-2 rounded-lg bg-surfaceMuted text-ink-900 text-[13px] hover:bg-ink-300")
                                .text(SVG_CALENDAR)
                                .text(" Lorem \u{2013} Ipsum ")
                                .text(SVG_CHEV)
                        })
                        .button(|b| {
                            b.class("h-9 px-3 inline-flex items-center gap-2 rounded-lg bg-surfaceMuted text-ink-900 text-[13px] hover:bg-ink-300")
                                .text("Sodales")
                        })
                })
        })
        // Outline
        .division(|d| {
            d.heading_3(|h| h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3").text("Outline"))
                .division(|g| {
                    g.class("flex flex-wrap items-center gap-3")
                        .button(|b| {
                            b.class("h-8 px-3 inline-flex items-center gap-2 rounded-lg border-[1.5px] border-ink-900 bg-surface text-ink-900 text-[13px] hover:bg-surfaceMuted")
                                .text("Omnis Vehicula ")
                                .text(SVG_CHEV)
                        })
                        .button(|b| {
                            b.class("h-9 px-3 rounded-lg border-[1.5px] border-ink-900 bg-surface text-ink-900 text-[13px] hover:bg-surfaceMuted")
                                .text("Dismiss")
                        })
                })
        })
        // Icon
        .division(|d| {
            d.heading_3(|h| h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3").text("Icon"))
                .division(|g| {
                    g.class("flex items-center gap-1")
                        .button(|b| b.class(ICON_BTN).text(SVG_FILTER))
                        .button(|b| b.class(ICON_BTN).text(SVG_SAVE))
                        .button(|b| b.class(ICON_BTN).text(SVG_UPLOAD))
                })
        })
        .build()
        .to_string();

    super::section(
        "buttons",
        "05",
        "Buttons",
        "Two variants: a soft gray fill or a 1.5px ink outline. The system reserves solid ink for typography only \u{2014} buttons are never pure black. Two heights: 32px (compact toolbars) and 36px (mobile / primary CTAs).",
        &content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render()));
    }
}
