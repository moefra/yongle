pub use yongle_digest::Digest;

/// The status of a blob in the CAS.
#[derive(Debug, Clone)]
pub struct BlobStat {
    pub digest: Digest,
    pub size: u64,
}
