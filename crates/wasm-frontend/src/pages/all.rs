//! All packages listing page.

// r[impl frontend.pages.all]

use html::inline_text::Anchor;
use html::text_content::Division;
use wasm_meta_registry_client::KnownPackage;

use crate::api_client::{ApiClient, ApiError};
use crate::layout;

/// Fetch all packages and render a paginated list.
pub(crate) async fn render(client: &ApiClient) -> String {
    match client.fetch_all_packages(0, 500).await {
        Ok(packages) => render_packages(&packages),
        Err(err) => render_error(&err),
    }
}

/// Render the package listing page.
fn render_packages(packages: &[KnownPackage]) -> String {
    let mut body = Division::builder();

    // Page header with count
    body.division(|div| {
        div.class("flex items-baseline justify-between pb-6 border-b border-border mb-6")
            .heading_1(|h1| {
                h1.class("text-3xl font-bold tracking-tight")
                    .text("All Packages")
            })
            .span(|s| {
                s.class("text-sm text-fg-faint")
                    .text(format!("{} packages", packages.len()))
            })
    });

    if packages.is_empty() {
        body.division(|div| {
            div.class("py-16 text-center").paragraph(|p| {
                p.class("text-fg-muted")
                    .text("No packages found. The registry may still be syncing.")
            })
        });
    } else {
        // Table-style header
        body.division(|div| {
            div.class("hidden sm:flex items-baseline gap-3 px-2 pb-2 text-xs text-fg-faint uppercase tracking-wide")
                .span(|s| s.class("w-48 shrink-0").text("Package"))
                .span(|s| s.class("w-20 shrink-0").text("Version"))
                .span(|s| s.text("Description"))
        });

        let mut list = Division::builder();
        list.class("divide-y divide-border-light");
        for pkg in packages {
            list.push(render_row(pkg));
        }
        body.push(list.build());
    }

    layout::document("All Packages", &body.build().to_string())
}

/// Render the page with an API error message.
fn render_error(err: &ApiError) -> String {
    let mut body = Division::builder();

    body.division(|div| {
        div.class("pb-6 border-b border-border mb-6")
            .heading_1(|h1| {
                h1.class("text-3xl font-bold tracking-tight")
                    .text("All Packages")
            })
    });

    body.division(|div| {
        div.class("py-16 text-center")
            .paragraph(|p| {
                p.class("text-fg font-semibold")
                    .text("Unable to load packages")
            })
            .paragraph(|p| p.class("text-sm text-fg-muted mt-2").text(err.to_string()))
    });

    layout::document("All Packages", &body.build().to_string())
}

/// Render a single package row.
fn render_row(pkg: &KnownPackage) -> Anchor {
    let display_name = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("{ns}:{name}"),
        _ => pkg.repository.clone(),
    };

    let href = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("/{ns}/{name}"),
        _ => "#".to_string(),
    };

    let description = pkg.description.as_deref().unwrap_or("");

    let version = pkg.tags.first().map_or("—", String::as_str);

    Anchor::builder()
        .href(href)
        .class(
            "flex items-baseline gap-3 py-3 hover:bg-surface -mx-2 px-2 rounded transition-colors",
        )
        .span(|s| {
            s.class("w-48 shrink-0 font-semibold text-accent truncate")
                .text(display_name)
        })
        .span(|s| {
            s.class("w-20 shrink-0 text-sm text-fg-faint")
                .text(version.to_owned())
        })
        .span(|s| {
            s.class("text-sm text-fg-muted truncate")
                .text(description.to_owned())
        })
        .build()
}
