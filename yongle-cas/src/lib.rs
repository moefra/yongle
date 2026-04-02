pub mod blob_range;
use async_trait::async_trait;
use std::{path::PathBuf, pin::Pin};
use thiserror::Error;
use tokio::io::AsyncRead;

use crate::blob_range::BlobRange;

#[derive(Error, Debug)]
pub enum CasError {}

/// A Content Addressable Storage (CAS) is a storage system that stores data by its content rather than by its location.
#[async_trait]
pub trait Cas: Send + Sync + 'static + std::fmt::Debug {}
