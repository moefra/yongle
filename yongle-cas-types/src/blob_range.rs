use rkyv::{Archive, Deserialize, Serialize};
use std::{
    num::NonZeroU64,
    ops::{RangeFrom, RangeFull},
};
use thiserror::Error;

/// Errors returned while constructing a [`BlobRange`].
#[derive(Error, Debug, PartialEq, Eq)]
pub enum BlobRangeError {
    /// The requested range length is zero, which is not supported for sized ranges.
    #[error("length is zero. input start {start:0} and length {length:?}")]
    ZeroLength { start: u64, length: u64 },
}

/// A byte range selector for reading blob content.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub enum BlobRange {
    /// Read the whole blob.
    Full,

    /// Read bytes in `[offset, offset + len)`.
    Slice { offset: u64, len: u64 },

    /// Read bytes in `[offset, end_of_blob)`.
    From { offset: u64 },

    /// Read the last `len` bytes of the blob.
    Suffix { len: u64 },
}

impl AsRef<BlobRange> for BlobRange {
    fn as_ref(&self) -> &BlobRange {
        self
    }
}

impl Default for BlobRange {
    fn default() -> Self {
        Self::full()
    }
}

impl BlobRange {
    /// Creates a blob range from an offset and an optional length.
    ///
    /// This compatibility constructor maps:
    /// - `(0, None)` to [`BlobRange::Full`]
    /// - `(offset > 0, None)` to [`BlobRange::From`]
    /// - `(offset, Some(len > 0))` to [`BlobRange::Slice`]
    ///
    /// Returns [`BlobRangeError::ZeroLength`] when `length` is `Some(0)`.
    #[inline]
    pub fn new(start: u64, length: Option<u64>) -> Result<Self, BlobRangeError> {
        match length {
            Some(length) => Self::slice(start, length),
            None if start == 0 => Ok(Self::Full),
            None => Ok(Self::From { offset: start }),
        }
    }

    /// Creates a range that reads the whole blob.
    pub fn full() -> Self {
        Self::Full
    }

    /// Creates a range that reads `len` bytes starting at `offset`.
    ///
    /// Returns [`BlobRangeError::ZeroLength`] when `len` is zero.
    pub fn slice(offset: u64, len: u64) -> Result<Self, BlobRangeError> {
        if len == 0 {
            return Err(BlobRangeError::ZeroLength {
                start: offset,
                length: len,
            });
        }

        Ok(Self::Slice { offset, len })
    }

    /// Creates a range that reads from `offset` to the end of the blob.
    ///
    /// An offset of zero is normalized to [`BlobRange::Full`].
    pub fn from_offset(offset: u64) -> Self {
        if offset == 0 {
            Self::Full
        } else {
            Self::From { offset }
        }
    }

    /// Creates a range that reads the last `len` bytes of the blob.
    ///
    /// Returns [`BlobRangeError::ZeroLength`] when `len` is zero.
    pub fn suffix(len: u64) -> Result<Self, BlobRangeError> {
        if len == 0 {
            return Err(BlobRangeError::ZeroLength {
                start: 0,
                length: len,
            });
        }

        Ok(Self::Suffix { len })
    }

    /// Returns the absolute offset when the range starts at a concrete position.
    pub fn offset(&self) -> Option<u64> {
        match self {
            Self::Slice { offset, .. } | Self::From { offset } => Some(*offset),
            Self::Full | Self::Suffix { .. } => None,
        }
    }

    /// Returns the requested length when the range has an explicit byte count.
    pub fn len(&self) -> Option<u64> {
        match self {
            Self::Slice { len, .. } | Self::Suffix { len } => Some(*len),
            Self::Full | Self::From { .. } => None,
        }
    }

    /// Returns the requested length as a non-zero value when present.
    pub fn len_nonzero(&self) -> Option<NonZeroU64> {
        self.len().and_then(NonZeroU64::new)
    }

    /// Returns `true` when the range reads the full blob.
    pub fn is_full(&self) -> bool {
        matches!(self, Self::Full)
    }

    /// Returns `true` when the range cannot fit within a blob of `span_length` bytes.
    pub fn is_out_of_span_length(&self, span_length: u64) -> bool {
        match self {
            Self::Full => false,
            Self::Slice { offset, len } => {
                *offset >= span_length
                    || offset
                        .checked_add(*len)
                        .is_none_or(|end| end > span_length)
            }
            Self::From { offset } => *offset >= span_length,
            Self::Suffix { len } => *len > span_length,
        }
    }
}

/// Supports `100..` style ranges.
impl From<RangeFrom<u64>> for BlobRange {
    fn from(r: RangeFrom<u64>) -> Self {
        Self::from_offset(r.start)
    }
}

/// Supports `..` as a whole-blob range.
impl From<RangeFull> for BlobRange {
    fn from(_: RangeFull) -> Self {
        Self::full()
    }
}

#[cfg(test)]
mod tests {
    use super::{BlobRange, BlobRangeError};
    use std::num::NonZeroU64;

    #[test]
    fn full_like_constructors_normalize_to_full() {
        assert_eq!(BlobRange::default(), BlobRange::Full);
        assert_eq!(BlobRange::full(), BlobRange::Full);
        assert_eq!(BlobRange::from(..), BlobRange::Full);
        assert_eq!(BlobRange::from(0..), BlobRange::Full);
        assert_eq!(BlobRange::from_offset(0), BlobRange::Full);
    }

    #[test]
    fn new_maps_to_expected_variants() {
        assert_eq!(BlobRange::new(0, None).unwrap(), BlobRange::Full);
        assert_eq!(
            BlobRange::new(12, None).unwrap(),
            BlobRange::From { offset: 12 }
        );
        assert_eq!(
            BlobRange::new(12, Some(4)).unwrap(),
            BlobRange::Slice { offset: 12, len: 4 }
        );
    }

    #[test]
    fn zero_length_constructors_return_errors() {
        assert_eq!(
            BlobRange::new(10, Some(0)).unwrap_err(),
            BlobRangeError::ZeroLength {
                start: 10,
                length: 0,
            }
        );
        assert_eq!(
            BlobRange::slice(10, 0).unwrap_err(),
            BlobRangeError::ZeroLength {
                start: 10,
                length: 0,
            }
        );
        assert_eq!(
            BlobRange::suffix(0).unwrap_err(),
            BlobRangeError::ZeroLength {
                start: 0,
                length: 0,
            }
        );
    }

    #[test]
    fn enum_accessors_match_variant_semantics() {
        let full = BlobRange::Full;
        assert_eq!(full.offset(), None);
        assert_eq!(full.len(), None);
        assert_eq!(full.len_nonzero(), None);
        assert!(full.is_full());

        let slice = BlobRange::Slice { offset: 3, len: 7 };
        assert_eq!(slice.offset(), Some(3));
        assert_eq!(slice.len(), Some(7));
        assert_eq!(slice.len_nonzero(), NonZeroU64::new(7));
        assert!(!slice.is_full());

        let from = BlobRange::From { offset: 5 };
        assert_eq!(from.offset(), Some(5));
        assert_eq!(from.len(), None);
        assert_eq!(from.len_nonzero(), None);
        assert!(!from.is_full());

        let suffix = BlobRange::Suffix { len: 9 };
        assert_eq!(suffix.offset(), None);
        assert_eq!(suffix.len(), Some(9));
        assert_eq!(suffix.len_nonzero(), NonZeroU64::new(9));
        assert!(!suffix.is_full());
    }

    #[test]
    fn span_length_checks_follow_variant_rules() {
        assert!(!BlobRange::Full.is_out_of_span_length(0));

        assert!(!BlobRange::Slice { offset: 2, len: 3 }.is_out_of_span_length(5));
        assert!(BlobRange::Slice { offset: 5, len: 1 }.is_out_of_span_length(5));
        assert!(BlobRange::Slice { offset: 4, len: 2 }.is_out_of_span_length(5));

        assert!(!BlobRange::From { offset: 4 }.is_out_of_span_length(5));
        assert!(BlobRange::From { offset: 5 }.is_out_of_span_length(5));

        assert!(!BlobRange::Suffix { len: 5 }.is_out_of_span_length(5));
        assert!(BlobRange::Suffix { len: 6 }.is_out_of_span_length(5));
    }

    #[test]
    fn slice_overflow_is_treated_as_out_of_span() {
        let range = BlobRange::Slice {
            offset: u64::MAX - 1,
            len: 4,
        };

        assert!(range.is_out_of_span_length(u64::MAX));
    }
}
