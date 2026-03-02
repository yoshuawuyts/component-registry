-- Rename table: wit_interface → wit_type
ALTER TABLE wit_interface RENAME TO wit_type;

-- Rename table: wit_interface_dependency → wit_type_dependency
ALTER TABLE wit_interface_dependency RENAME TO wit_type_dependency;

-- Rename column: wit_world.wit_interface_id → wit_type_id
ALTER TABLE wit_world RENAME COLUMN wit_interface_id TO wit_type_id;

-- Rename column: wit_world_import.resolved_interface_id → resolved_type_id
ALTER TABLE wit_world_import RENAME COLUMN resolved_interface_id TO resolved_type_id;

-- Rename column: wit_world_export.resolved_interface_id → resolved_type_id
ALTER TABLE wit_world_export RENAME COLUMN resolved_interface_id TO resolved_type_id;

-- Rename column: wit_type_dependency.resolved_interface_id → resolved_type_id
ALTER TABLE wit_type_dependency RENAME COLUMN resolved_interface_id TO resolved_type_id;

-- Recreate indexes with new names and column references.
-- Drop old indexes first, then create new ones.

DROP INDEX IF EXISTS uq_wit_interface;
CREATE UNIQUE INDEX uq_wit_type ON wit_type(
    package_name,
    COALESCE(version, ''),
    COALESCE(oci_layer_id, -1)
);

DROP INDEX IF EXISTS uq_wit_interface_dependency;
CREATE UNIQUE INDEX uq_wit_type_dependency ON wit_type_dependency(
    dependent_id,
    declared_package,
    COALESCE(declared_version, '')
);

DROP INDEX IF EXISTS idx_wit_iface_name_version;
CREATE INDEX idx_wit_type_name_version ON wit_type(package_name, version);

DROP INDEX IF EXISTS idx_wit_iface_provenance;
CREATE INDEX idx_wit_type_provenance ON wit_type(oci_manifest_id);

DROP INDEX IF EXISTS idx_world_import_resolved;
CREATE INDEX idx_world_import_resolved ON wit_world_import(resolved_type_id);

DROP INDEX IF EXISTS idx_world_export_resolved;
CREATE INDEX idx_world_export_resolved ON wit_world_export(resolved_type_id);

DROP INDEX IF EXISTS idx_wit_dep_declared;
CREATE INDEX idx_wit_dep_declared ON wit_type_dependency(declared_package, declared_version);

DROP INDEX IF EXISTS idx_wit_dep_resolved;
CREATE INDEX idx_wit_dep_resolved ON wit_type_dependency(resolved_type_id);
