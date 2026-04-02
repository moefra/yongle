pub mod blob_range;
use async_trait::async_trait;
use std::{path::PathBuf, pin::Pin};
use thiserror::Error;
use tokio::io::AsyncRead;
use yongle_digest::blake3::Blake3Hash;

use crate::blob_range::BlobRange;

pub enum HashAlgorithm {
    Blake3(Blake3Hash),
}

pub struct Digest {
    pub algo: HashAlgorithm,
    pub bytes: Box<[u8]>,
}

pub struct BlobId(pub Digest);

#[derive(Error, Debug)]
pub enum CasError {}

/// A Content Addressable Storage (CAS) is a storage system that stores data by its content rather than by its location.
#[async_trait]
pub trait Cas: Send + Sync + 'static + std::fmt::Debug {}
