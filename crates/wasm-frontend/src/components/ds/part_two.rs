//! Part Two — Components divider.

use html::text_content::Division;

/// Render the components divider section.
pub(crate) fn render() -> String {
    let divider = Division::builder()
        .class("mt-24 mb-2 flex items-baseline gap-3")
        .span(|s| {
            s.class("text-[11px] mono uppercase tracking-wider text-ink-500")
                .text("Part Two")
        })
        .span(|s| {
            s.class("h-px flex-1 bg-line-soft")
                .style("background:var(--c-line-soft)")
        })
        .build()
        .to_string();

    let h2 = html::content::Heading2::builder()
        .class("text-[28px] md:text-[32px] font-semibold tracking-tight")
        .text("Components")
        .build()
        .to_string();

    let p = html::text_content::Paragraph::builder()
        .class("mt-2 max-w-2xl text-[14px] text-ink-700 leading-relaxed")
        .text("Composed patterns built from the foundations above. Each component documents its anchor markup and the variants it supports.")
        .build()
        .to_string();

    format!(r#"{divider}{h2}{p}<div class="mt-6 border-t rule"></div>"#,)
}

#[cfg(test)]
const SNAPSHOT: &str = r##"

    <!-- ============================== -->
    <!-- COMPONENTS DIVIDER             -->
    <!-- ============================== -->
    <div class="mt-24 mb-2 flex items-baseline gap-3">
      <span class="text-[11px] mono uppercase tracking-wider text-ink-500">Part Two</span>
      <span class="h-px flex-1 bg-line-soft" style="background:var(--c-line-soft)"></span>
    </div>
    <h2 class="text-[28px] md:text-[32px] font-semibold tracking-tight">Components</h2>
    <p class="mt-2 max-w-2xl text-[14px] text-ink-700 leading-relaxed">
      Composed patterns built from the foundations above. Each component documents
      its anchor markup and the variants it supports.
    </p>
    <div class="mt-6 border-t rule"></div>

    <!-- ============================== -->
"##;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::ds::normalize_html;

    #[test]
    fn part_two_matches_snapshot() {
        assert_eq!(normalize_html(&render()), normalize_html(SNAPSHOT));
    }
}
