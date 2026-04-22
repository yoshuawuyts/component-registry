//! World detail page.

use crate::components::ds::page_header;
use crate::components::ds::wit_item::{self, TypeTag, WitItem, WitItemKind};
use crate::wit_doc::{WitDocument, WorldDoc, WorldItemDoc};
use html::text_content::Division;
use wasm_meta_registry_client::{KnownPackage, PackageVersion};

use super::package_shell;

/// Render the world detail page.
#[must_use]
pub(crate) fn render(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    world: &WorldDoc,
    doc: &WitDocument,
) -> String {
    let display_name = package_shell::display_name_for(pkg);
    let title = format!("{display_name} \u{2014} {}", world.name);

    let header = page_header::page_header_block(
        &format!("v{version} \u{00b7} World"),
        &world.name,
        world.docs.as_deref().unwrap_or("No description available."),
        None,
    )
    .to_string();

    let mut content = Division::builder();
    content.class("space-y-10 pt-8");

    // Build a doc lookup from the API's enriched world data (has cross-package docs).
    let api_docs = build_api_doc_lookup(version_detail, &world.name);

    if !world.imports.is_empty() {
        content.push(render_item_section("Imports", &world.imports, &api_docs));
    }
    if !world.exports.is_empty() {
        content.push(render_item_section("Exports", &world.exports, &api_docs));
    }

    let body_html = content.build().to_string();

    let nav = super::sidebar::render_sidebar(&super::sidebar::SidebarContext {
        display_name: &display_name,
        version,
        versions: &pkg.tags,
        doc: Some(doc),
        components: version_detail.map_or(&[][..], |d| &d.components),
        url_base: &package_shell::url_base_for(pkg, version),
        active: super::sidebar::SidebarActive::World(&world.name),
        annotations: version_detail.and_then(|d| d.annotations.as_ref()),
        kind_label: package_shell::kind_label_for(pkg),
        description: pkg.description.as_deref(),
        registry: &pkg.registry,
        repository: &pkg.repository,
        digest: version_detail.map(|d| d.digest.as_str()),
    });

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
        nav_html: Some(nav.to_string()),
    };
    package_shell::render_page_with_crumbs(
        &ctx,
        &title,
        &header,
        &body_html,
        &[crate::components::ds::breadcrumb::Crumb {
            label: world.name.clone(),
            href: None,
        }],
        None,
    )
}

/// Build a lookup map of interface name → doc string from the API's enriched
/// world data. This provides cross-package docs that the WIT parser can't.
fn build_api_doc_lookup(
    version_detail: Option<&PackageVersion>,
    world_name: &str,
) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let Some(detail) = version_detail else {
        return map;
    };
    for world in &detail.worlds {
        if world.name != world_name {
            continue;
        }
        for iface in world.imports.iter().chain(world.exports.iter()) {
            if let Some(docs) = &iface.docs {
                let mut key = iface.package.clone();
                if let Some(name) = &iface.interface {
                    key.push('/');
                    key.push_str(name);
                }
                map.insert(key, docs.clone());
            }
        }
    }
    map
}

/// Render an imports or exports section, grouped by package namespace.
fn render_item_section(
    heading: &str,
    items: &[WorldItemDoc],
    api_docs: &std::collections::HashMap<String, String>,
) -> Division {
    let rows: Vec<WitItem> = items
        .iter()
        .map(|item| match item {
            WorldItemDoc::Interface { name, url, docs } => {
                let name_no_ver = strip_version(name);
                let desc = docs.clone().or_else(|| api_docs.get(name_no_ver).cloned());
                WitItem {
                    kind: WitItemKind::Interface,
                    name: name_no_ver.to_owned(),
                    href: url.clone().unwrap_or_default(),
                    docs: desc,
                    meta: String::new(),
                    deprecated: false,
                    id: None,
                }
            }
            WorldItemDoc::Function(func) => WitItem {
                kind: WitItemKind::Function,
                name: func.name.clone(),
                href: func.url.clone(),
                docs: func.docs.clone(),
                meta: String::new(),
                deprecated: false,
                id: None,
            },
            WorldItemDoc::Type(ty) => WitItem {
                kind: WitItemKind::Type(TypeTag::from_kind(&ty.kind)),
                name: ty.name.clone(),
                href: ty.url.clone(),
                docs: ty.docs.clone(),
                meta: String::new(),
                deprecated: false,
                id: None,
            },
        })
        .collect();

    wit_item::render_item_section(heading, &rows)
}

/// Strip version suffix from a qualified name.
///
/// `"wasi:cli/environment@0.2.11"` → `"wasi:cli/environment"`
fn strip_version(name: &str) -> &str {
    name.split('@').next().unwrap_or(name)
}

/// Extract the short interface name from a fully-qualified WIT name.
///
/// `"wasi:http/types@0.2.11"` → `"types"`
#[allow(dead_code)]
fn short_interface_name(name: &str) -> String {
    let without_version = name.split('@').next().unwrap_or(name);
    without_version
        .rsplit('/')
        .next()
        .unwrap_or(without_version)
        .to_owned()
}
