# Manifest

## Dependencies Structure

r[manifest.dependencies]
The manifest MUST support a `[dependencies]` table with `components` and
`interfaces` sub-tables for organizing dependency types.

## Manifest Parsing

r[manifest.parse.compact]
The manifest parser MUST support compact dependency notation where the
value is a version string (e.g., `"1.0.0"`).

r[manifest.parse.explicit]
The manifest parser MUST support explicit table dependency notation with
registry, namespace, package, and version fields.

r[manifest.parse.empty]
The manifest parser MUST handle empty manifest files.

r[manifest.parse.mixed]
The manifest parser MUST support manifests with both `components` and
`interfaces` sections under `[dependencies]`.

r[manifest.parse.all-dependencies]
Iterating all dependencies MUST yield both component and interface entries.

r[manifest.parse.permissions]
The manifest parser MUST support sandbox permissions in explicit format
dependencies.

r[manifest.parse.no-permissions]
Dependencies without permissions MUST still parse correctly.

## Version Semantics

r[manifest.version.semver-default]
Bare version strings (e.g., `"1.0.0"`) MUST be treated as Cargo-style
caret requirements (`^1.0.0`), matching versions >=1.0.0 and <2.0.0.

r[manifest.version.semver-pre-1]
Pre-1.0 versions MUST use narrower compatibility ranges: `"0.2.3"` matches
>=0.2.3 and <0.3.0; `"0.0.3"` matches >=0.0.3 and <0.0.4.

r[manifest.version.explicit-operators]
The manifest parser MUST support explicit version operators including
`>=`, `~`, `=`, `*`, and comma-separated ranges (e.g., `">=1.0, <2.0"`).

r[manifest.version.special-values]
Empty strings and `"latest"` MUST be treated as wildcard requirements
matching any version.

r[manifest.version.invalid]
Invalid version strings MUST be rejected with a clear error message.

r[manifest.dependency.version-accessor]
The `Dependency` type MUST provide access to the version string from
both compact and explicit formats.

## Manifest Serialization

r[manifest.serialize.compact]
The manifest serializer MUST produce valid TOML in compact format.

r[manifest.serialize.explicit]
The manifest serializer MUST produce valid TOML in explicit format.
