//! Error page shown when the registry API is unreachable.

use html::text_content::Division;

use crate::components::{link_button, page_heading};
use crate::layout;

/// Render an error page with a description of what went wrong.
#[must_use]
pub(crate) fn render(message: &str) -> String {
    let body = Division::builder()
        .class("text-center py-20")
        .heading_1(|h1| {
            h1.class(format!("{} text-ink-900", page_heading::H1_CLASS))
                .text("Something went wrong")
        })
        .paragraph(|p| {
            p.class("text-[13px] text-ink-500 mt-4")
                .text(message.to_owned())
        })
        .division(|d| {
            d.class("mt-8").push(link_button::render(
                link_button::Variant::Primary,
                "/",
                "Go to Home",
            ))
        })
        .build();

    layout::document_with_nav("Error", &body.to_string())
}
