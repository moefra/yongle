//! Identifier types used across Yongle crates.
//!
//! The CAS types were defined in the [`yongle-cas-types`] crate.

use std::fmt;

use url::Url;

/// Represents a semantic name.
///
/// The semantic name is presented as a URL.
pub enum SemanticName {
    Type(TypeUrl),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SemanticError {
    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("invalid scheme of url, expected '{0}': {1}")]
    MismatchScheme(&'static str, String),
    #[error("Require a url authority/hostname but not present")]
    MissingAuthority,
    #[error("Require a url path with at least two segments but not present")]
    MissingPath,
    #[error("Require a url path without empty segments but not present")]
    EmptyPathSegment,
    #[error("Require a url without {0} but {0} is detected")]
    RedundantUrlComponent(&'static str),
    #[error(
        "Invalid version at the last url path segment: {0}, expected `v[u64 number]` e.g. `v1`
    "
    )]
    InvalidVersion(String),
}

macro_rules! define_semantic_url {
    ($id:ident, $scheme:literal) => {
        impl $id {
            /// Create a new semantic url from the given arguments.
            pub fn new(name: &str, domain: &str, version: u64) -> Result<Self, SemanticError> {
                let raw = format!("{0}://{domain}/{name}/v{version}", $scheme);
                let url = Url::parse(&raw)?;
                Self::from_url(url)
            }

            /// The semantic url.
            pub fn url(&self) -> &Url {
                &self.0
            }

            /// Parse a semantic url from a string.
            ///
            /// Also see [`from_url`](Self::from_url)
            pub fn from_string(url: &str) -> Result<Self, SemanticError> {
                let url = Url::parse(url)?;
                Self::from_url(url)
            }

            /// Validate a url then saved as semantic url.
            ///
            /// It only allows urls with the scheme `$scheme`.
            ///
            /// The url must have a host authority and no query string.
            ///
            /// The url must have no fragment.
            ///
            /// The url must have a path with length greater or equals than 2.
            ///
            /// The last path segment of url must match the `v{number}` pattern.
            ///
            /// The number should can be parsed as
            /// [`u64 with 10 base`](https://doc.rust-lang.org/std/primitive.u64.html#method.from_str_radix)
            pub fn from_url(url: Url) -> Result<Self, SemanticError> {
                if url.scheme() != $scheme {
                    return Err(SemanticError::MismatchScheme(
                        $scheme,
                        url.scheme().to_string(),
                    ));
                }

                if url.host_str().is_none() {
                    return Err(SemanticError::MissingAuthority);
                }

                if url.query().is_some() {
                    return Err(SemanticError::RedundantUrlComponent("query"));
                }

                if url.fragment().is_some() {
                    return Err(SemanticError::RedundantUrlComponent("fragment"));
                }

                if url.password().is_some() {
                    return Err(SemanticError::RedundantUrlComponent("password"));
                }

                if !url.username().is_empty() {
                    return Err(SemanticError::RedundantUrlComponent("username"));
                }

                let segments: Vec<_> = url
                    .path_segments()
                    .ok_or(SemanticError::MissingPath)?
                    .collect();

                // at least have: /<name>/v<version>
                if segments.len() < 2 {
                    return Err(SemanticError::MissingPath);
                }

                // last segment must be v<number>
                let version_segment = segments.last().unwrap();
                let version_str = version_segment
                    .strip_prefix('v')
                    .ok_or_else(|| SemanticError::InvalidVersion(version_segment.to_string()))?;

                if version_str.is_empty() || u64::from_str_radix(version_str, 10).is_err() {
                    return Err(SemanticError::InvalidVersion(version_segment.to_string()));
                }

                if segments[..segments.len() - 1]
                    .iter()
                    .any(|segment| segment.is_empty())
                {
                    return Err(SemanticError::EmptyPathSegment);
                }

                Ok(Self(url))
            }

            pub fn version(&self) -> u64 {
                let segments: Vec<_> = self.0.path_segments().unwrap().collect();
                let version_segment = segments.last().unwrap();
                version_segment.strip_prefix('v').unwrap().parse().unwrap()
            }
        }

        impl std::str::FromStr for $id {
            type Err = SemanticError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_string(s)
            }
        }

        impl AsRef<Url> for $id {
            fn as_ref(&self) -> &Url {
                &self.0
            }
        }
    };
}

/// A Url with `type` semantic scheme.
///
/// It's also a scheme for object. For example, `type://yl.kawayi.moe/tag/v1`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeUrl(Url);

define_semantic_url!(TypeUrl, "type");

macro_rules! define_id {
    ($id:ident) => {
        /// Stable and unique identifier.
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            rkyv::Serialize,
            rkyv::Deserialize,
            rkyv::Archive,
            serde::Deserialize,
            serde::Serialize,
        )]
        pub struct $id(u128);

        impl $id {
            /// Create a new unique identifier,
            /// using random bytes.
            pub fn new_random() -> Self {
                let mut buf = [0u8; 16];
                getrandom::fill(&mut buf).unwrap();
                Self(u128::from_le_bytes(buf))
            }
            pub fn raw(&self) -> u128 {
                self.0
            }
        }

        impl fmt::Display for $id {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "0x{:X}", self.0)
            }
        }
    };
}

define_id!(SourceId);
define_id!(ObjectId);
define_id!(ActorId);
