//! Pagination component for list pages.
//!
//! Enabled and disabled button class strings for prev/next and page
//! number controls.

/// Class string for an enabled pagination button.
pub(crate) const BUTTON_CLASS: &str =
    "px-3 py-1.5 border border-line text-[13px] hover:bg-surfaceMuted transition-colors";

/// Class string for a disabled pagination button.
pub(crate) const DISABLED_CLASS: &str =
    "px-3 py-1.5 border border-line-light text-[13px] text-ink-400";
