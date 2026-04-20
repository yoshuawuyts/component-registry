//! 10 — Tooltip.

use html::text_content::Division;

/// Render the tooltip section.
pub(crate) fn render() -> String {
    let content = Division::builder()
        .class("p-12 bg-canvas border border-line rounded-lg flex items-center justify-center")
        .division(|tip| {
            tip.class("shadow-tooltip rounded-md backdrop-blur text-canvas px-3 py-2 text-[12px] leading-tight")
                .style("background: color-mix(in oklab, var(--c-ink-900) 85%, transparent);")
                .division(|lbl| {
                    lbl.class("text-ink-300")
                        .text("Cycle 14 \u{00b7} Aenean")
                })
                .division(|row| {
                    row.class("mt-1 flex items-center justify-between gap-6")
                        .span(|s| s.text("Maxima:"))
                        .span(|s| s.class("font-medium").text("9.42"))
                })
                .division(|row| {
                    row.class("flex items-center justify-between gap-6")
                        .span(|s| s.text("Minima:"))
                        .span(|s| s.class("font-medium").text("3.18"))
                })
        })
        .build()
        .to_string();

    super::section(
        "tooltip",
        "10",
        "Tooltip",
        "Inverted surface with backdrop blur. Caption label above, key/value rows with right-aligned medium values.",
        &content,
    )
}

#[cfg(test)]
const SNAPSHOT: &str = r##"
    <section id="tooltip" class="pt-12 md:pt-16">
      <div class="grid md:grid-cols-[200px_1fr] gap-6 md:gap-12">
        <div>
          <div class="text-[12px] mono uppercase tracking-wider text-ink-500">10</div>
          <h2 class="mt-2 text-[24px] font-semibold tracking-tight">Tooltip</h2>
          <p class="mt-2 text-[13px] text-ink-500 leading-relaxed">
            Inverted surface with backdrop blur. Caption label above, key/value rows
            with right-aligned medium values.
          </p>
        </div>
        <div class="p-12 bg-canvas border border-line rounded-lg flex items-center justify-center">
          <div class="shadow-tooltip rounded-md backdrop-blur text-canvas px-3 py-2 text-[12px] leading-tight"
            style="background: color-mix(in oklab, var(--c-ink-900) 85%, transparent);">
            <div class="text-ink-300">Cycle 14 · Aenean</div>
            <div class="mt-1 flex items-center justify-between gap-6"><span>Maxima:</span><span
                class="font-medium">9.42</span></div>
            <div class="flex items-center justify-between gap-6"><span>Minima:</span><span
                class="font-medium">3.18</span></div>
          </div>
        </div>
      </div>
    </section>
"##;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::ds::normalize_html;

    #[test]
    fn matches_snapshot() {
        assert_eq!(normalize_html(&render()), normalize_html(SNAPSHOT));
    }
}
