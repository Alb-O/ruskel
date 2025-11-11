//! Test utilities shared across test modules.

use rustdoc_types::{Abi, FunctionHeader, Generics};

/// Create an empty Generics instance for testing.
pub fn empty_generics() -> Generics {
	Generics {
		params: Vec::new(),
		where_predicates: Vec::new(),
	}
}

/// Create a default FunctionHeader for testing.
pub fn default_header() -> FunctionHeader {
	FunctionHeader {
		is_const: false,
		is_unsafe: false,
		is_async: false,
		abi: Abi::Rust,
	}
}

/// Load a rustdoc JSON fixture from the tests/fixtures directory.
///
/// # Example
///
/// ```ignore
/// let crate_data = load_fixture("widget_crate");
/// ```
#[cfg(test)]
pub fn load_fixture(name: &str) -> rustdoc_types::Crate {
	use std::path::PathBuf;

	let manifest_dir = env!("CARGO_MANIFEST_DIR");
	let fixture_path = PathBuf::from(manifest_dir)
		.join("tests")
		.join("fixtures")
		.join(name)
		.join("rustdoc.json");

	let json = std::fs::read_to_string(&fixture_path)
		.unwrap_or_else(|_| panic!("Failed to read fixture at {}", fixture_path.display()));

	serde_json::from_str(&json)
		.unwrap_or_else(|e| panic!("Failed to parse fixture JSON: {}", e))
}
