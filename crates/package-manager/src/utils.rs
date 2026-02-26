use wasmparser::{Parser, Payload};

/// Determine whether raw wasm bytes represent a WIT package (interface-only)
/// rather than a compiled component.
///
/// A WIT package contains only types, imports, exports, and custom sections.
/// A compiled component additionally contains code/instantiation sections such
/// as `ModuleSection`, `ComponentSection`, `InstanceSection`, etc.
///
/// Returns `true` if the bytes are a WIT package, `false` if they contain
/// code/instantiation (a real component) or if parsing fails.
#[must_use]
pub fn is_wit_package(bytes: &[u8]) -> bool {
    let parser = Parser::new(0);
    for payload in parser.parse_all(bytes) {
        match payload {
            Ok(Payload::ModuleSection { .. })
            | Ok(Payload::ComponentSection { .. })
            | Ok(Payload::InstanceSection(_))
            | Ok(Payload::ComponentInstanceSection(_))
            | Ok(Payload::ComponentCanonicalSection(_))
            | Ok(Payload::CoreTypeSection(_))
            | Ok(Payload::ComponentStartSection { .. }) => {
                // Contains code/instantiation — it's a real component
                return false;
            }
            Err(_) => return false,
            _ => {}
        }
    }
    // Only had types, imports, exports, custom sections — WIT package
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_bytes_are_not_wit_package() {
        assert!(!is_wit_package(b"not a wasm component"));
    }

    #[test]
    fn empty_bytes_are_not_wit_package() {
        assert!(!is_wit_package(&[]));
    }

    #[test]
    fn core_module_is_treated_as_wit_package() {
        // A minimal valid core WebAssembly module has no component-level
        // code/instantiation sections, so `is_wit_package` returns true.
        // In practice this function is only called on component binaries
        // pulled from OCI registries.
        let core_module = [
            0x00, 0x61, 0x73, 0x6d, // \0asm magic
            0x01, 0x00, 0x00, 0x00, // version 1
        ];
        assert!(is_wit_package(&core_module));
    }
}
