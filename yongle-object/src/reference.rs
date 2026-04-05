use yongle_cas_types::BlobId;
use yongle_id::{ObjectId, TypeUrl};

/// An indirect reference to an object, blob, or type.
///
/// The `indirect` means it's value should be resolved in a `id -> object` map when access the real object.
///
/// The reference can not be used for gc:it do not have a strength relationship with the object it references.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndirectRef {
    Object(ObjectId),
    Blob(BlobId),
    Type(TypeUrl),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefKind {
    Strong,
    Weak,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefEdge {
    pub reference: IndirectRef,
    pub kind: RefKind,
}
