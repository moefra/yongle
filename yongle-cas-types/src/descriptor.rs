use yongle_digest::Digest;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Descriptor {
    pub digest: Digest,
    pub size: u64,
}
