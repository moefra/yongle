//! Text primitive types.

/// Small immutable string optimized for short values.
///
/// Prefer this for identifiers, field names, tags, and other
/// frequently cloned short strings.
pub use ::smol_str::SmolStr;

/// Builder for `SmolStr`.
pub use ::smol_str::SmolStrBuilder;

/// Formatting macro for `SmolStr`.
pub use ::smol_str::format_smolstr;

/// Clone-on-write string with inline storage.
///
/// Prefer this for shared text values that may occasionally
/// be mutated or incrementally built.
pub use ::ecow::EcoString;

/// Clone-on-write vector with shared backing storage.
pub use ::ecow::EcoVec;
