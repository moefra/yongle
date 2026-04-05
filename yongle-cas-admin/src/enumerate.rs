use yongle_cas_types::BlobId;

/// This trait provides a way to enumerate CAS objects.
pub trait CasEnumerator {
    fn enumerate(&self) -> impl Iterator<Item = BlobId>;
}
