//! Structural digest helpers shared across Yongle crates.

#![deny(missing_docs)]

/// Structural BLAKE3 hashing helpers.
pub mod blake3;
/// Structural SHA-256 hashing helpers.
pub mod sha256;

/// A generic digest type that contains a hash value.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Digest {
    /// A BLAKE3 hash.
    Blake3(blake3::Hash),
    /// A SHA-256 hash.
    Sha256(sha256::Hash),
}
