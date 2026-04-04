pub mod blob_range;
pub mod descriptor;
pub mod error;
pub mod stat;

pub use blob_range::BlobRange;
pub use descriptor::Descriptor;
pub use stat::BlobStat;

use yongle_digest::Digest;

/// A blob identifier, consisting of a digest as unique identifier.
pub struct BlobId(pub Digest);
