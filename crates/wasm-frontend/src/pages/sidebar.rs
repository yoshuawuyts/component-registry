//! Shared sidebar components for detail pages.
//!
//! Provides a navigation sidebar showing sibling interfaces/worlds and
//! package metadata, using the DS nested sidebar component (C01).

use crate::components::ds::sidebar::{self, SidebarEntry, SidebarGroup, SidebarItem};
use crate::wit_doc::WitDocument;
use html::content::Aside;
use wasm_meta_registry_client::OciAnnotations;

/// GitHub logo SVG icon (14px, ink-500).
const SVG_GITHUB: &str = r#"<svg class="h-3.5 w-3.5 text-ink-500 flex-shrink-0" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true"><path d="M8 .2a8 8 0 0 0-2.5 15.6c.4 0 .55-.17.55-.38v-1.4c-2.22.48-2.69-1.07-2.69-1.07-.36-.92-.89-1.17-.89-1.17-.73-.5.05-.49.05-.49.8.06 1.23.83 1.23.83.71 1.23 1.87.87 2.33.66.07-.52.28-.87.5-1.07-1.77-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.83-2.15-.08-.2-.36-1.02.08-2.13 0 0 .67-.22 2.2.82A7.6 7.6 0 0 1 8 4.04c.68 0 1.37.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.11.16 1.93.08 2.13.52.56.83 1.28.83 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.74.54 1.49v2.21c0 .21.15.46.55.38A8 8 0 0 0 8 .2Z" /></svg>"#;

/// Lucide book-open icon (14px, ink-500).
const SVG_BOOK: &str = concat!(
    r#"<svg class="h-3.5 w-3.5 text-ink-500 flex-shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">"#,
    include_str!("../../../../vendor/lucide/book-open.svg"),
    "</svg>"
);

/// Lucide house icon (14px, ink-500).
const SVG_HOUSE: &str = concat!(
    r#"<svg class="h-3.5 w-3.5 text-ink-500 flex-shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">"#,
    include_str!("../../../../vendor/lucide/house.svg"),
    "</svg>"
);

/// Lucide scale icon (14px, ink-500).
const SVG_SCALE: &str = concat!(
    r#"<svg class="h-3.5 w-3.5 text-ink-500 flex-shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">"#,
    include_str!("../../../../vendor/lucide/scale.svg"),
    "</svg>"
);

/// Context needed to render the detail page sidebar.
pub(crate) struct SidebarContext<'a> {
    /// The package display name (e.g. `"wasi:cli"`).
    pub display_name: &'a str,
    /// The current version string.
    pub version: &'a str,
    /// All available version tags (newest first).
    pub versions: &'a [String],
    /// The parsed WIT document for navigation links.
    pub doc: &'a WitDocument,
    /// Which sidebar item is currently active.
    pub active: SidebarActive<'a>,
    /// OCI annotations for the current version (optional).
    pub annotations: Option<&'a OciAnnotations>,
}

/// Which item in the sidebar is currently active.
pub(crate) enum SidebarActive<'a> {
    /// An interface page (name of the interface).
    Interface(&'a str),
    /// An item within an interface (interface name, item name).
    Item(&'a str, #[allow(dead_code)] &'a str),
    /// A world page (name of the world).
    World(&'a str),
}

/// Render the sidebar for a detail page using the DS nested sidebar.
pub(crate) fn render_sidebar(ctx: &SidebarContext<'_>) -> Aside {
    let pkg_url = format!("/{}/{}", ctx.display_name.replace(':', "/"), ctx.version);

    let mut items: Vec<SidebarItem> = Vec::new();

    // Package root entry
    items.push(SidebarItem::Entry(SidebarEntry {
        sigil_bg: "var(--c-cat-slate)",
        sigil_color: "var(--c-cat-slate-ink)",
        sigil_text: "\u{00b7}",
        name: ctx.display_name.to_owned(),
        href: pkg_url,
        meta: String::new(),
        active: false,
    }));

    // Worlds — each world is a group with its imports/exports as children
    for world in &ctx.doc.worlds {
        let is_active = matches!(ctx.active, SidebarActive::World(name) if name == world.name);
        let mut children = Vec::new();
        for item in world.imports.iter().chain(world.exports.iter()) {
            if let crate::wit_doc::WorldItemDoc::Interface {
                name,
                url: Some(url),
                ..
            } = item
            {
                children.push(SidebarEntry {
                    sigil_bg: "var(--c-cat-lilac)",
                    sigil_color: "var(--c-cat-lilac-ink)",
                    sigil_text: "I",
                    name: short_interface_name(name),
                    href: url.clone(),
                    meta: String::new(),
                    active: false,
                });
            }
        }
        items.push(SidebarItem::Group(SidebarGroup {
            label: world.name.clone(),
            href: Some(world.url.clone()),
            sigil_bg: Some("var(--c-cat-green)"),
            sigil_color: Some("var(--c-cat-green-ink)"),
            sigil_text: Some("W"),
            open: is_active,
            count: None,
            children,
        }));
    }

    // Interfaces — each interface is a group with its types and functions as children
    for iface in &ctx.doc.interfaces {
        let is_active = matches!(
            ctx.active,
            SidebarActive::Interface(name) if name == iface.name
        ) || matches!(
            ctx.active,
            SidebarActive::Item(iface_name, _) if iface_name == iface.name
        );
        let mut children = Vec::new();
        for ty in &iface.types {
            children.push(SidebarEntry {
                sigil_bg: "var(--c-cat-blue)",
                sigil_color: "var(--c-cat-blue-ink)",
                sigil_text: "T",
                name: ty.name.clone(),
                href: ty.url.clone(),
                meta: String::new(),
                active: matches!(
                    ctx.active,
                    SidebarActive::Item(_, item_name) if item_name == ty.name
                ),
            });
        }
        for func in &iface.functions {
            children.push(SidebarEntry {
                sigil_bg: "var(--c-cat-green)",
                sigil_color: "var(--c-cat-green-ink)",
                sigil_text: "f",
                name: func.name.clone(),
                href: func.url.clone(),
                meta: String::new(),
                active: matches!(
                    ctx.active,
                    SidebarActive::Item(_, item_name) if item_name == func.name
                ),
            });
        }
        items.push(SidebarItem::Group(SidebarGroup {
            label: iface.name.clone(),
            href: Some(iface.url.clone()),
            sigil_bg: Some("var(--c-cat-lilac)"),
            sigil_color: Some("var(--c-cat-lilac-ink)"),
            sigil_text: Some("I"),
            open: is_active,
            count: None,
            children,
        }));
    }

    let version_strs: Vec<&str> = ctx.versions.iter().map(String::as_str).collect();
    let base_url = format!("/{}/", ctx.display_name.replace(':', "/"));
    let project_html = build_project_section(ctx.annotations);
    let version_html = sidebar::render_version_selector(ctx.version, &version_strs, &base_url);
    let items_html = sidebar::render_items_nav(Some("Items"), &items);

    let mut aside = Aside::builder();
    aside.class("space-y-4");
    if let Some(version) = &version_html {
        aside.text(version.clone());
    }
    if let Some(project) = &project_html {
        aside.text(project.clone());
    }
    aside.text(items_html);
    aside.build()
}

/// Build a "Project" footer section from OCI annotations.
///
/// Uses tree-link rows with icons matching the DS C01 "Project" section:
/// GitHub logo for github.com/ghcr.io URLs, book for docs, house for homepage.
fn build_project_section(annotations: Option<&OciAnnotations>) -> Option<String> {
    let ann = annotations?;

    let mut rows = Vec::new();

    // Link rows — use icons matching the DS demo pattern
    if let Some(source) = &ann.source {
        let (icon, label) = icon_and_label_for_url(source, "Source");
        rows.push(project_link(source, icon, label));
    }
    if let Some(docs) = &ann.documentation {
        let (icon, label) = icon_and_label_for_url(docs, "Documentation");
        // Default to book icon unless it's a GitHub URL
        let icon = if is_github_url(docs) { icon } else { SVG_BOOK };
        rows.push(project_link(docs, icon, label));
    }
    if let Some(url) = &ann.url {
        // Only show if different from source
        if ann.source.as_deref() != Some(url) {
            let (icon, label) = icon_and_label_for_url(url, "Homepage");
            let icon = if is_github_url(url) { icon } else { SVG_HOUSE };
            rows.push(project_link(url, icon, label));
        }
    }

    // Metadata rows
    if let Some(license) = &ann.licenses {
        let base = strip_with_clause(license);
        rows.push(project_icon_row(SVG_SCALE, &base));
    }
    if let Some(authors) = &ann.authors {
        rows.push(detail_row("Authors", authors));
    }
    if let Some(vendor) = &ann.vendor {
        rows.push(detail_row("Vendor", vendor));
    }

    if rows.is_empty() {
        return None;
    }

    let items = rows.join("");
    Some(format!(
        r#"<div class="pb-4 mb-4 border-b-[1.5px] border-rule"><div class="mono uppercase tracking-wider text-[10px] text-ink-500 mb-2">Project</div><nav class="space-y-px">{items}</nav></div>"#
    ))
}

/// Render a project link as a tree-link with an icon and label.
fn project_link(href: &str, icon: &str, label: &str) -> String {
    format!(
        r#"<a href="{href}" class="tree-link" target="_blank" rel="noopener">{icon} {label}</a>"#
    )
}

/// Render a non-link row with an icon and text (tree-link styling, no href).
fn project_icon_row(icon: &str, text: &str) -> String {
    format!(r#"<div class="tree-link">{icon} {text}</div>"#)
}

/// Render a key-value detail row for the project section.
fn detail_row(label: &str, value: &str) -> String {
    format!(
        r#"<div class="flex items-baseline justify-between gap-4 py-1 text-[12px]"><span class="text-ink-500">{label}</span><span class="text-ink-700 mono text-right truncate">{value}</span></div>"#
    )
}

/// Check if a URL points to GitHub or GHCR.
fn is_github_url(url: &str) -> bool {
    url.contains("github.com") || url.contains("ghcr.io")
}

/// Return the appropriate icon and label for a URL.
///
/// GitHub/GHCR URLs get the GitHub logo; others get a house icon.
fn icon_and_label_for_url<'a>(url: &str, fallback_label: &'a str) -> (&'static str, &'a str) {
    let icon = if is_github_url(url) {
        SVG_GITHUB
    } else {
        SVG_HOUSE
    };
    (icon, fallback_label)
}

/// Extract the short interface name from a fully-qualified WIT name.
///
/// `"wasi:http/types@0.2.11"` → `"types"`
/// `"types"` → `"types"`
fn short_interface_name(name: &str) -> String {
    let without_version = name.split('@').next().unwrap_or(name);
    let short = without_version
        .rsplit('/')
        .next()
        .unwrap_or(without_version);
    short.to_owned()
}

/// Strip a `WITH` clause from an SPDX license expression.
///
/// `"Apache-2.0 WITH LLVM-Exception"` → `"Apache-2.0"`
/// `"MIT"` → `"MIT"`
fn strip_with_clause(license: &str) -> String {
    match license.find(" WITH ") {
        Some(pos) => license[..pos].to_owned(),
        None => license.to_owned(),
    }
}
