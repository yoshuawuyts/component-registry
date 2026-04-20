//! Table of contents.

use html::content::Navigation;

/// TOC entries: (href, label).
const ENTRIES: &[(&str, &str)] = &[
    ("#colors", "01 \u{2014} Color"),
    ("#typography", "02 \u{2014} Typography"),
    ("#spacing", "03 \u{2014} Spacing & Radius"),
    ("#elevation", "04 \u{2014} Elevation"),
    ("#buttons", "05 \u{2014} Buttons"),
    ("#tabs", "06 \u{2014} Tabs & Pills"),
    ("#nav", "07 \u{2014} Navigation"),
    ("#code", "08 \u{2014} Code Samples"),
    ("#bars", "09 \u{2014} Labels"),
    ("#tooltip", "10 \u{2014} Tooltip"),
    ("#table", "11 \u{2014} Table"),
    ("#icons", "12 \u{2014} Icons"),
    ("#fields", "13 \u{2014} Form Fields"),
    (
        "#toggles",
        "14 \u{2014} Checkbox \u{00b7} Radio \u{00b7} Switch",
    ),
    ("#badges", "15 \u{2014} Badges"),
    ("#dropdown", "16 \u{2014} Dropdown"),
    ("#modal", "17 \u{2014} Modal"),
    ("#breadcrumb", "18 \u{2014} Breadcrumb & Pagination"),
    ("#progress", "19 \u{2014} Progress & Spinner"),
    ("#empty", "20 \u{2014} Empty State"),
    ("#grid", "21 \u{2014} Grid"),
    ("#regions", "22 \u{2014} Regions"),
    ("#motion", "23 \u{2014} Motion"),
    ("#details", "24 \u{2014} Details"),
];

/// TOC entries for composed components.
const COMPONENT_ENTRIES: &[(&str, &str)] = &[
    ("#c-sidebar", "C01 \u{2014} Nested Sidebar"),
    ("#c-toc", "C02 \u{2014} On This Page"),
    ("#c-page-header", "C03 \u{2014} Page Header"),
    ("#c-item-list", "C04 \u{2014} Item List"),
    ("#c-item-details", "C05 \u{2014} Item Details"),
    ("#c-navbar", "C06 \u{2014} Navbar"),
];

/// Render the table of contents.
pub(crate) fn render() -> String {
    let mut nav = Navigation::builder();
    nav.class("py-6 grid grid-flow-col grid-rows-12 md:grid-rows-6 gap-y-2 gap-x-6 text-[13px]");
    for (href, label) in ENTRIES {
        nav.anchor(|a| {
            a.href(*href)
                .class("text-ink-700 hover:text-ink-900")
                .text(*label)
        });
    }
    nav.span(|s| {
        s.class("text-ink-400 mono uppercase tracking-wider text-[11px] mt-2")
            .text("Components")
    });
    for (href, label) in COMPONENT_ENTRIES {
        nav.anchor(|a| {
            a.href(*href)
                .class("text-ink-700 hover:text-ink-900")
                .text(*label)
        });
    }
    nav.build().to_string()
}

#[cfg(test)]
const SNAPSHOT: &str = r##"
    <nav class="py-6 grid grid-flow-col grid-rows-12 md:grid-rows-6 gap-y-2 gap-x-6 text-[13px]">
      <a href="#colors" class="text-ink-700 hover:text-ink-900">01 — Color</a>
      <a href="#typography" class="text-ink-700 hover:text-ink-900">02 — Typography</a>
      <a href="#spacing" class="text-ink-700 hover:text-ink-900">03 — Spacing & Radius</a>
      <a href="#elevation" class="text-ink-700 hover:text-ink-900">04 — Elevation</a>
      <a href="#buttons" class="text-ink-700 hover:text-ink-900">05 — Buttons</a>
      <a href="#tabs" class="text-ink-700 hover:text-ink-900">06 — Tabs & Pills</a>
      <a href="#nav" class="text-ink-700 hover:text-ink-900">07 — Navigation</a>
      <a href="#code" class="text-ink-700 hover:text-ink-900">08 — Code Samples</a>
      <a href="#bars" class="text-ink-700 hover:text-ink-900">09 — Labels</a>
      <a href="#tooltip" class="text-ink-700 hover:text-ink-900">10 — Tooltip</a>
      <a href="#table" class="text-ink-700 hover:text-ink-900">11 — Table</a>
      <a href="#icons" class="text-ink-700 hover:text-ink-900">12 — Icons</a>
      <a href="#fields" class="text-ink-700 hover:text-ink-900">13 — Form Fields</a>
      <a href="#toggles" class="text-ink-700 hover:text-ink-900">14 — Checkbox · Radio · Switch</a>
      <a href="#badges" class="text-ink-700 hover:text-ink-900">15 — Badges</a>
      <a href="#dropdown" class="text-ink-700 hover:text-ink-900">16 — Dropdown</a>
      <a href="#modal" class="text-ink-700 hover:text-ink-900">17 — Modal</a>
      <a href="#breadcrumb" class="text-ink-700 hover:text-ink-900">18 — Breadcrumb & Pagination</a>
      <a href="#progress" class="text-ink-700 hover:text-ink-900">19 — Progress & Spinner</a>
      <a href="#empty" class="text-ink-700 hover:text-ink-900">20 — Empty State</a>
      <a href="#grid" class="text-ink-700 hover:text-ink-900">21 — Grid</a>
      <a href="#regions" class="text-ink-700 hover:text-ink-900">22 — Regions</a>
      <a href="#motion" class="text-ink-700 hover:text-ink-900">23 — Motion</a>
      <a href="#details" class="text-ink-700 hover:text-ink-900">24 — Details</a>
      <span class="text-ink-400 mono uppercase tracking-wider text-[11px] mt-2">Components</span>
      <a href="#c-sidebar" class="text-ink-700 hover:text-ink-900">C01 — Nested Sidebar</a>
      <a href="#c-toc" class="text-ink-700 hover:text-ink-900">C02 — On This Page</a>
      <a href="#c-page-header" class="text-ink-700 hover:text-ink-900">C03 — Page Header</a>
      <a href="#c-item-list" class="text-ink-700 hover:text-ink-900">C04 — Item List</a>
      <a href="#c-item-details" class="text-ink-700 hover:text-ink-900">C05 — Item Details</a>
      <a href="#c-navbar" class="text-ink-700 hover:text-ink-900">C06 — Navbar</a>
    </nav>
"##;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::ds::normalize_html;

    #[test]
    fn toc_matches_snapshot() {
        assert_eq!(normalize_html(&render()), normalize_html(SNAPSHOT));
    }
}
