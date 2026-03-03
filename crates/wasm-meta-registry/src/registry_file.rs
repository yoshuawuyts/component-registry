//! Per-namespace registry file parsing.

use serde::Deserialize;

use crate::config::{PackageKind, PackageSource};

/// A per-namespace registry file.
///
/// Each file defines a single namespace with its OCI registry base path,
/// plus zero or more `[[component]]` and `[[interface]]` entries.
///
/// # Example
///
/// ```toml
/// [namespace]
/// name = "wasi"
/// registry = "ghcr.io/webassembly"
///
/// [[interface]]
/// name = "io"
/// repository = "wasi/io"
/// ```
#[derive(Debug, Clone, Deserialize)]
#[must_use]
pub struct RegistryFile {
    /// The namespace definition.
    pub namespace: Namespace,
    /// Runnable Wasm components in this namespace.
    #[serde(default)]
    pub component: Vec<PackageEntry>,
    /// WIT interface type packages in this namespace.
    #[serde(default)]
    pub interface: Vec<PackageEntry>,
}

/// A WIT namespace mapped to an OCI registry base path.
#[derive(Debug, Clone, Deserialize)]
#[must_use]
pub struct Namespace {
    /// WIT namespace name (must match the filename).
    pub name: String,
    /// OCI registry base path (e.g., "ghcr.io/webassembly").
    pub registry: String,
}

/// A package entry within a namespace.
#[derive(Debug, Clone, Deserialize)]
#[must_use]
pub struct PackageEntry {
    /// Package name under the namespace (e.g., "io" for `wasi:io`).
    pub name: String,
    /// OCI repository path, relative to the namespace's registry.
    pub repository: String,
}

impl RegistryFile {
    /// Parse a per-namespace registry TOML string.
    ///
    /// # Errors
    ///
    /// Returns an error if the TOML is invalid or missing required fields.
    pub fn from_toml(toml_str: &str) -> anyhow::Result<Self> {
        let file: RegistryFile = toml::from_str(toml_str)?;
        Ok(file)
    }

    /// Convert this registry file into a list of [`PackageSource`] entries.
    #[must_use]
    pub fn into_package_sources(self) -> Vec<PackageSource> {
        let registry = self.namespace.registry;
        let mut sources = Vec::new();
        for entry in self.component {
            sources.push(PackageSource {
                registry: registry.clone(),
                repository: entry.repository,
                name: entry.name,
                kind: PackageKind::Component,
            });
        }
        for entry in self.interface {
            sources.push(PackageSource {
                registry: registry.clone(),
                repository: entry.repository,
                name: entry.name,
                kind: PackageKind::Interface,
            });
        }
        sources
    }
}
