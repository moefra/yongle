use async_trait::async_trait;
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncWrite};
use yongle_cas_types::{BlobId, BlobStat, Descriptor, error::CasError};
use yongle_digest::Digest;

/// This trait is the combination of `AsyncRead` and `AsyncWrite`.
pub trait ReadAndWrite: AsyncRead + AsyncWrite {}
impl<T: AsyncRead + AsyncWrite> ReadAndWrite for T {}

pub type PinnedRead = Pin<Box<dyn AsyncRead + Send + 'static>>;
pub type PinnedWrite = Pin<Box<dyn AsyncWrite + Send + 'static>>;
pub type PinnedIo = Pin<Box<dyn ReadAndWrite + Send + 'static>>;
type Result<T> = ::std::result::Result<T, CasError>;

#[derive(Debug, Clone)]
pub struct ReadRequest {
    id: BlobId,
    range: Option<(u64, u64)>,
}

#[async_trait]
pub trait CasRead {
    /// Returns the blob stat if it exists in the CAS.
    async fn stat(&self, id: &BlobId) -> Result<Option<BlobStat>>;
    /// Opens the blob for reading.
    async fn open(&self, id: &BlobId) -> Result<PinnedRead>;
}

#[async_trait]
pub trait CasWrite {
    /// Put a byte slice into the CAS, returning a descriptor.
    ///
    /// - `excepted` provides a way to validate the bytes before storing them.
    ///    If validation fails, the stream is not stored and return error.
    /// - `bytes` is the byte slice to store.
    ///
    /// Returns a `Descriptor` on success.
    async fn put_bytes(&self, expected: Option<&Digest>, bytes: &[u8]) -> Result<Descriptor>;
    /// Put a stream into the CAS, returning a descriptor.
    ///
    /// - `excepted` provides a way to validate the bytes before storing them.
    ///    If validation fails, the stream is not stored and return error.
    /// - `reader` is the stream to store.
    ///
    /// Returns a `Descriptor` on success.
    async fn put_stream(&self, expected: Option<&Digest>, reader: PinnedRead)
    -> Result<Descriptor>;
}

/// This trait is the combination of `CasReade` and `CasWrite`.
pub trait CasReadAndWrite: CasRead + CasWrite {}
impl<T: CasRead + CasWrite> CasReadAndWrite for T {}

pub type PinnedCasReader = Pin<Box<dyn CasRead + Send + 'static>>;
pub type PinnedCasWrite = Pin<Box<dyn CasWrite + Send + 'static>>;
pub type PinnedCasIo = Pin<Box<dyn CasReadAndWrite + Send + 'static>>;
