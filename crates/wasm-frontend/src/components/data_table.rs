//! Data table component.
//!
//! Provides class constants and builders for the standard data tables
//! used on WIT item detail pages (record fields, variant cases, enum
//! cases, flags).

use html::tables::TableRow;

/// Class string for the table container.
pub(crate) const TABLE_CLASS: &str = "w-full text-[13px]";

/// Class string for the header row.
pub(crate) const HEADER_ROW_CLASS: &str = "border-b border-line text-left text-ink-500";

/// Class string for a header cell (non-last).
pub(crate) const HEADER_CELL_CLASS: &str = "py-2 pr-4 font-medium";

/// Class string for the last header cell (no right padding).
pub(crate) const HEADER_CELL_LAST_CLASS: &str = "py-2 font-medium";

/// Class string for a data row.
pub(crate) const ROW_CLASS: &str = "border-b-2 border-line";

/// Class string for a name/identifier cell (monospace, accent color).
pub(crate) const NAME_CELL_CLASS: &str = "py-2 pr-4 font-mono text-accent";

/// Class string for a value cell (monospace, ink-900).
pub(crate) const VALUE_CELL_CLASS: &str = "py-2 pr-4 font-mono text-ink-900";

/// Class string for a description cell (last column).
pub(crate) const DESC_CELL_CLASS: &str = "py-2 text-ink-700";

/// Build a 2-column header row (name, description).
pub(crate) fn header_2(col1: &str, col2: &str) -> TableRow {
    TableRow::builder()
        .class(HEADER_ROW_CLASS)
        .table_header(|th| th.class(HEADER_CELL_CLASS).text(col1.to_owned()))
        .table_header(|th| th.class(HEADER_CELL_LAST_CLASS).text(col2.to_owned()))
        .build()
}

/// Build a 3-column header row (name, type/payload, description).
pub(crate) fn header_3(col1: &str, col2: &str, col3: &str) -> TableRow {
    TableRow::builder()
        .class(HEADER_ROW_CLASS)
        .table_header(|th| th.class(HEADER_CELL_CLASS).text(col1.to_owned()))
        .table_header(|th| th.class(HEADER_CELL_CLASS).text(col2.to_owned()))
        .table_header(|th| th.class(HEADER_CELL_LAST_CLASS).text(col3.to_owned()))
        .build()
}

/// Build a 2-column data row (name, description).
pub(crate) fn row_2(name: &str, description: &str) -> TableRow {
    TableRow::builder()
        .class(ROW_CLASS)
        .table_cell(|td| td.class(NAME_CELL_CLASS).text(name.to_owned()))
        .table_cell(|td| td.class(DESC_CELL_CLASS).text(description.to_owned()))
        .build()
}
