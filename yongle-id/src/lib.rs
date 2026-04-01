//! Identifier types used across Yongle crates.

/// Qualified identifier parsing and validation support.
pub mod qualified;

/// A fully-qualified identifier composed of domain, semantic version, and path.
pub use qualified::QualifiedName;
/// Errors returned when parsing a [`QualifiedName`] from text.
pub use qualified::QualifiedNameParseError;
