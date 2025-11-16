use std::fmt;

use serde_json::Error as SerdeError;

/// Aggregate errors produced by the ripdoc-core API.
#[derive(Debug)]
pub enum RipdocError {
	/// Errors returned by cargo/target resolution helpers.
	Cargo(ripdoc_cargo::RipdocError),
	/// Errors emitted while rendering skeleton output.
	Render(ripdoc_render::error::RipdocError),
	/// Failed to encode or decode JSON.
	Serialization(SerdeError),
	/// Invalid target specifications provided by the user.
	InvalidTarget(String),
}

impl fmt::Display for RipdocError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Cargo(err) => write!(f, "{err}"),
			Self::Render(err) => write!(f, "{err}"),
			Self::Serialization(err) => write!(f, "{err}"),
			Self::InvalidTarget(message) => write!(f, "{message}"),
		}
	}
}

impl std::error::Error for RipdocError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::Cargo(err) => Some(err),
			Self::Render(err) => Some(err),
			Self::Serialization(err) => Some(err),
			Self::InvalidTarget(_) => None,
		}
	}
}

impl From<ripdoc_cargo::RipdocError> for RipdocError {
	fn from(err: ripdoc_cargo::RipdocError) -> Self {
		Self::Cargo(err)
	}
}

impl From<ripdoc_render::error::RipdocError> for RipdocError {
	fn from(err: ripdoc_render::error::RipdocError) -> Self {
		Self::Render(err)
	}
}

impl From<SerdeError> for RipdocError {
	fn from(err: SerdeError) -> Self {
		Self::Serialization(err)
	}
}

/// Result type returned by the ripdoc-core library.
pub type Result<T> = std::result::Result<T, RipdocError>;
