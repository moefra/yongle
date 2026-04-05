use yongle_cas_types::BlobId;
use yongle_id::{ActorId, ObjectId, TypeUrl};

#[derive(
    Debug,
    Clone,
    PartialEq,
    rkyv::Deserialize,
    rkyv::Serialize,
    rkyv::Archive,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum PropertyValue {
    Bool(bool),
    Number(u64),
    Float(f64),
    String(String),
    Object(ObjectId),
    Blob(BlobId),
    Actor(ActorId),
}

pub type PropertyMap = yongle_primitives::FastMap<TypeUrl, PropertyValue>;
