use thiserror::Error;
use yongle_digest::Digest;

#[derive(Error, Debug)]
pub enum CasError {
    #[error("blob not found")]
    NotFound,
    #[error("blob already exists")]
    AlreadyExists,
    #[error("blob size mismatch: expected {0}, got {1}")]
    SizeMismatch(u64, u64),
    #[error("blob digest mismatch: expected {0:?}, got {1:?}")]
    DigestMismatch(Digest, Digest),
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("other error")]
    Other(#[from] eyre::Error),
}
