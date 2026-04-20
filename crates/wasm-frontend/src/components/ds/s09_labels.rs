//! 09 — Labels.

use html::text_content::Division;

/// Label bar entries: (bg class, ink class, text).
const BARS: &[(&str, &str, &str)] = &[
    ("bg-cat-blue", "text-cat-blueInk", "Lorem ipsum dolor"),
    ("bg-cat-pink", "text-cat-pinkInk", "Sit amet"),
    ("bg-cat-cream", "text-cat-creamInk", "Consectetur"),
    ("bg-cat-green", "text-cat-greenInk", "Adipiscing elit"),
    ("bg-cat-peach", "text-cat-peachInk", "Sed do eiusmod"),
    ("bg-cat-lilac", "text-cat-lilacInk", "Tempor incididunt"),
    ("bg-cat-teal", "text-cat-tealInk", "Ut labore"),
    ("bg-cat-rust", "text-cat-rustInk", "Et dolore magna"),
    ("bg-cat-plum", "text-cat-plumInk", "Aliqua enim"),
    ("bg-cat-slate", "text-cat-slateInk", "Ad minim veniam"),
];

/// Render the labels section.
pub(crate) fn render() -> String {
    let mut col = Division::builder();
    col.class("flex flex-col items-start gap-2");
    for (bg, ink, text) in BARS {
        col.division(|d| d.class(format!("bar {bg} {ink}")).text(*text));
    }
    let content = col.build().to_string();

    super::section(
        "bars",
        "09",
        "Labels",
        "28px tall, 6px radius, label inset 12px. Pastel fill with paired ink for text \u{2014} 4.5:1 contrast minimum.",
        &content,
    )
}

#[cfg(test)]
const SNAPSHOT: &str = r##"
    <section id="bars" class="pt-12 md:pt-16">
      <div class="grid md:grid-cols-[200px_1fr] gap-6 md:gap-12">
        <div>
          <div class="text-[12px] mono uppercase tracking-wider text-ink-500">09</div>
          <h2 class="mt-2 text-[24px] font-semibold tracking-tight">Labels</h2>
          <p class="mt-2 text-[13px] text-ink-500 leading-relaxed">
            28px tall, 6px radius, label inset 12px. Pastel fill with paired ink for
            text — 4.5:1 contrast minimum.
          </p>
        </div>
        <div class="flex flex-col items-start gap-2">
          <div class="bar bg-cat-blue text-cat-blueInk">Lorem ipsum dolor</div>
          <div class="bar bg-cat-pink text-cat-pinkInk">Sit amet</div>
          <div class="bar bg-cat-cream text-cat-creamInk">Consectetur</div>
          <div class="bar bg-cat-green text-cat-greenInk">Adipiscing elit</div>
          <div class="bar bg-cat-peach text-cat-peachInk">Sed do eiusmod</div>
          <div class="bar bg-cat-lilac text-cat-lilacInk">Tempor incididunt</div>
          <div class="bar bg-cat-teal text-cat-tealInk">Ut labore</div>
          <div class="bar bg-cat-rust text-cat-rustInk">Et dolore magna</div>
          <div class="bar bg-cat-plum text-cat-plumInk">Aliqua enim</div>
          <div class="bar bg-cat-slate text-cat-slateInk">Ad minim veniam</div>
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
