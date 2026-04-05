pub mod blob_range;
pub mod descriptor;
pub mod error;
pub mod stat;

pub use blob_range::BlobRange;
pub use descriptor::Descriptor;
pub use stat::BlobStat;

use yongle_digest::Digest;

/// A blob identifier, consisting of a digest as unique identifier.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct BlobId(pub Digest);
