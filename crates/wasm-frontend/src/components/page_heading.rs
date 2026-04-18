//! Page heading component.
//!
//! Standardized headings used on all top-level pages (search, all,
//! namespace, downloads, docs, about, error).

/// Class string for the display-sized page heading used on item detail pages.
pub(crate) const DISPLAY_CLASS: &str = "text-[28px] sm:text-[36px] font-semibold tracking-tight";

/// Class string for the primary page heading (h1).
pub(crate) const H1_CLASS: &str = "text-[28px] font-semibold tracking-tight";

/// Class string for a page sub-heading (h2).
pub(crate) const H2_CLASS: &str = "text-[22px] font-semibold tracking-tight mt-10 mb-4";

/// Class string for a subtitle / count line below the heading.
pub(crate) const SUBTITLE_CLASS: &str = "text-[13px] text-ink-500 mt-2";

/// Class string for body text paragraphs on content pages.
pub(crate) const BODY_CLASS: &str = "text-ink-700 leading-relaxed";
