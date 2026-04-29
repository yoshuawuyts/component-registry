//! Dynamic dispatch for "library-style" components.
//!
//! A library-style component is one that exports plain functions or
//! interfaces but does not target either the `wasi:cli/command` world
//! (no `wasi:cli/run` export) or the `wasi:http/proxy` world (no
//! `wasi:http/incoming-handler` export).
//!
//! For these, `component run` translates each WIT export into a
//! `clap` sub-command, parses arguments dynamically, invokes the
//! function via wasmtime's untyped component API, and prints the
//! result to stdout.
//!
//! This module is split into three concerns:
//!
//! - [`wit`] — extract a [`LibrarySurface`] from component bytes.
//! - [`cli`] — build a [`clap::Command`] from a `LibrarySurface` and
//!   parse user input into an [`Invocation`].
//! - [`render`] — print [`wasmtime::component::Val`] results to
//!   stdout / stderr.
//!
//! The actual wasmtime invocation lives in `component-cli-internal-run`.

pub(crate) mod cli;
pub(crate) mod render;
pub(crate) mod wit;

pub(crate) use cli::{build_clap, parse_invocation};
pub(crate) use render::print_results;
pub(crate) use wit::{LibraryExtractError, extract_library_surface};
