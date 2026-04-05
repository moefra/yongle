use smol_str::SmolStr;
use yongle_cas_types::BlobId;
use yongle_id::{ObjectId, TypeUrl};

pub mod map;
pub mod property;
pub mod reference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadRef {
    pub schema: TypeUrl,
    pub blob: BlobId,
}

#[derive(Debug, Clone)]
pub struct ObjectRecord {
    pub id: ObjectId,
    pub kind: property::ObjectKind,
    pub export_class: property::ExportClass,
    pub source: property::ObjectSource,
    pub owner: property::ObjectOwner,
    pub refs: Vec<reference::RefEdge>,
    pub payload: Option<PayloadRef>,
    pub properties: map::PropertyMap,
}
