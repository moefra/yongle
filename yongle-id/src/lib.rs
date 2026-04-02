//! Identifier types used across Yongle crates.

/// Qualified identifier parsing and validation support.
pub mod qualified;

/// A fully-qualified identifier composed of domain, semantic version, and path.
pub use qualified::QualifiedName;
/// Errors returned when parsing a [`QualifiedName`] from text.
pub use qualified::QualifiedNameParseError;

/// Unique stable actor identifier.
///
/// Identifies a human, AI agent, automation, or external system.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct ActorId(u128);

/// Unique stable internal identifier for a domain entity.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct EntityId(u128);

/// Stable internal identifier for a source object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(u128);
