use std::{
    array::TryFromSliceError,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

use camino::{Utf8Path, Utf8PathBuf};
use rkyv::{Archive, Deserialize, Serialize};
use sha2::Digest as _;

/// Structural hashing into a SHA-256 digest.
///
/// Implementations must preserve the current encoding rules used by `yongle-digest`
/// so equivalent values hash identically across all supported environments.
pub trait Sha256Hash {
    /// Feeds this value into the provided SHA-256 hasher using structural encoding.
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256);

    /// Computes the SHA-256 digest for this value.
    fn get_sha256(&self) -> Hash {
        let mut hasher = sha2::Sha256::new();
        self.hash_into_sha256(&mut hasher);
        let hash_bytes: [u8; 32] = hasher.finalize().into();
        Hash::new(hash_bytes)
    }
}

macro_rules! impl_sha256_for_tuple {
    ($($ty:ident),*) => {
        impl<$($ty: Sha256Hash),*> Sha256Hash for ($($ty,)*) {
            fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
                hasher.update(b"::std::tuple::Tuple");
                let len = [0u8; 0].len() $( + { let _ = stringify!($ty); 1 } )*;
                (len as u64).hash_into_sha256(hasher);

                #[allow(non_snake_case)]
                let ($($ty,)*) = &self;
                $($ty.hash_into_sha256(hasher);)*
            }
        }
    };
}

impl<T> Sha256Hash for PhantomData<T> {
    fn hash_into_sha256(&self, _hasher: &mut sha2::Sha256) {
        // PhantomData does not contribute bytes to the structural digest.
    }
}

impl_sha256_for_tuple!(A);
impl_sha256_for_tuple!(A, B);
impl_sha256_for_tuple!(A, B, C);
impl_sha256_for_tuple!(A, B, C, D);
impl_sha256_for_tuple!(A, B, C, D, E);
impl_sha256_for_tuple!(A, B, C, D, E, F);
impl_sha256_for_tuple!(A, B, C, D, E, F, G);
impl_sha256_for_tuple!(A, B, C, D, E, F, G, H);
impl_sha256_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_sha256_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_sha256_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_sha256_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

impl<T: Sha256Hash + ?Sized> Sha256Hash for &T {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        (**self).hash_into_sha256(hasher)
    }
}

impl<T: Sha256Hash + ?Sized> Sha256Hash for &mut T {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        (**self).hash_into_sha256(hasher)
    }
}

impl Sha256Hash for smol_str::SmolStr {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::smol_str::SmolStr");
        self.len().hash_into_sha256(hasher);
        hasher.update(self.as_bytes());
    }
}

impl Sha256Hash for ecow::EcoString {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::ecow::EcoString");
        self.len().hash_into_sha256(hasher);
        hasher.update(self.as_bytes());
    }
}

impl<T: Sha256Hash> Sha256Hash for ecow::EcoVec<T> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::ecow::EcoVec");
        self.len().hash_into_sha256(hasher);

        for value in self.iter() {
            value.hash_into_sha256(hasher);
        }
    }
}

impl Sha256Hash for String {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::string::String");
        self.len().hash_into_sha256(hasher);
        hasher.update(self.as_bytes());
    }
}

impl<T: Sha256Hash> Sha256Hash for &[T] {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::slice::Slice");
        self.len().hash_into_sha256(hasher);

        for value in self.iter() {
            value.hash_into_sha256(hasher);
        }
    }
}

impl<T: Sha256Hash, const N: usize> Sha256Hash for [T; N] {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::array::Array");
        self.len().hash_into_sha256(hasher);

        for value in self.iter() {
            value.hash_into_sha256(hasher);
        }
    }
}

impl<'a, T: Sha256Hash + Clone> Sha256Hash for std::borrow::Cow<'a, T> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        self.as_ref().hash_into_sha256(hasher);
    }
}

impl Sha256Hash for str {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::str::Str");
        self.len().hash_into_sha256(hasher);
        hasher.update(self.as_bytes());
    }
}

impl Sha256Hash for Utf8PathBuf {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::camino::Utf8PathBuf");
        self.as_str().hash_into_sha256(hasher);
    }
}

impl Sha256Hash for Utf8Path {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::camino::Utf8Path");
        self.as_str().hash_into_sha256(hasher);
    }
}

impl<T: Sha256Hash + ?Sized> Sha256Hash for Rc<T> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        (**self).hash_into_sha256(hasher)
    }
}

impl<T: Sha256Hash + ?Sized> Sha256Hash for Arc<T> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        (**self).hash_into_sha256(hasher)
    }
}

impl<T: Sha256Hash + ?Sized> Sha256Hash for Box<T> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        (**self).hash_into_sha256(hasher)
    }
}

impl<T: Sha256Hash + Sized + Ord + Eq> Sha256Hash for Vec<T> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::vec::Vec");
        self.len().hash_into_sha256(hasher);

        for value in self.iter() {
            value.hash_into_sha256(hasher);
        }
    }
}

impl<T: Sha256Hash, V: Sha256Hash> Sha256Hash for BTreeMap<T, V> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::collections::BTreeMap");
        self.len().hash_into_sha256(hasher);

        for pair in self.iter() {
            pair.0.hash_into_sha256(hasher);
            pair.1.hash_into_sha256(hasher);
        }
    }
}

impl<T: Sha256Hash> Sha256Hash for BTreeSet<T> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::collections::BTreeSet");
        self.len().hash_into_sha256(hasher);

        for value in self.iter() {
            value.hash_into_sha256(hasher);
        }
    }
}

impl<T: Sha256Hash + Sized + Ord + std::hash::Hash + Eq, V: Sha256Hash, S> Sha256Hash
    for HashMap<T, V, S>
{
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::collections::HashMap");
        self.len().hash_into_sha256(hasher);

        let mut pairs = self.iter().collect::<Vec<_>>();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        for pair in pairs.into_iter() {
            pair.0.hash_into_sha256(hasher);
            pair.1.hash_into_sha256(hasher);
        }
    }
}

impl<T: Sha256Hash + Sized + Ord + std::hash::Hash + Eq, S> Sha256Hash for HashSet<T, S> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::collections::HashSet");
        self.len().hash_into_sha256(hasher);

        let mut items = self.iter().collect::<Vec<_>>();
        items.sort();

        for value in items.into_iter() {
            value.hash_into_sha256(hasher);
        }
    }
}

impl<T: Sha256Hash + Sized> Sha256Hash for Option<T> {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(b"::std::option::Option");
        match self {
            Some(value) => {
                hasher.update(&[1u8]);
                value.hash_into_sha256(hasher);
            }
            None => {
                hasher.update(&[0u8]);
                ().hash_into_sha256(hasher);
            }
        };
    }
}

/// Structural magic bytes used when hashing the unit value `()`.
pub const UNIT_HASH_MAGIC: u128 = 0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF;

impl Sha256Hash for () {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(UNIT_HASH_MAGIC.to_le_bytes());
    }
}

macro_rules! impl_sha256_hash_for_fixed_numbers {
    ($($t:ty),*) => {
        $(
            impl Sha256Hash for $t {
                fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
                    hasher.update(self.to_le_bytes());
                }
            }
        )*
    }
}

impl_sha256_hash_for_fixed_numbers!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl Sha256Hash for char {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        (*self as u32).hash_into_sha256(hasher);
    }
}

impl Sha256Hash for f32 {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        let val = if self.is_nan() {
            0x7FC0_0000_u32
        } else if *self == 0.0 {
            0_u32
        } else {
            self.to_bits()
        };
        hasher.update(val.to_le_bytes());
    }
}

impl Sha256Hash for f64 {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        let val = if self.is_nan() {
            0x7FF8_0000_0000_0000_u64
        } else if *self == 0.0_f64 {
            0_u64
        } else {
            self.to_bits()
        };
        hasher.update(val.to_le_bytes());
    }
}

macro_rules! impl_sha256_hash_for_arch_numbers {
    ($($t:ty),*) => {
        $(
            impl Sha256Hash for $t {
                fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
                    (*self as u64).hash_into_sha256(hasher)
                }
            }
        )*
    }
}

impl_sha256_hash_for_arch_numbers!(usize, isize);

impl Sha256Hash for bool {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        let val = if *self { 1u8 } else { 0u8 };
        hasher.update([val]);
    }
}

/// A stable 32-byte SHA-256 digest value.
#[derive(
    Clone,
    Hash,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Archive,
    Serialize,
    Deserialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[rkyv(derive(Hash, Eq, PartialEq))]
pub struct Hash {
    hash_bytes: [u8; 32],
}

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex = self.to_hex();
        f.debug_struct("Hash")
            .field("sha256", &hex.as_str())
            .finish()
    }
}

impl Hash {
    /// Creates a digest wrapper from the provided bytes.
    pub fn new(hash_bytes: [u8; 32]) -> Self {
        Self { hash_bytes }
    }

    /// Creates a digest wrapper by copying bytes from a fixed-size array reference.
    pub fn from_bytes(hash_bytes: &[u8; 32]) -> Self {
        Self {
            hash_bytes: *hash_bytes,
        }
    }

    /// Returns the underlying digest bytes.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.hash_bytes
    }

    /// Returns the lowercase hexadecimal encoding of this digest.
    pub fn to_hex(&self) -> arrayvec::ArrayString<64> {
        let mut hex_string = arrayvec::ArrayString::new();
        hex_string.push_str(&hex::encode(self.hash_bytes));
        hex_string
    }
}

impl From<[u8; 32]> for Hash {
    fn from(value: [u8; 32]) -> Self {
        Hash { hash_bytes: value }
    }
}

impl From<&[u8; 32]> for Hash {
    fn from(value: &[u8; 32]) -> Self {
        Hash { hash_bytes: *value }
    }
}

impl TryFrom<&[u8]> for Hash {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Hash {
            hash_bytes: value.try_into()?,
        })
    }
}

impl From<Hash> for [u8; 32] {
    fn from(value: Hash) -> Self {
        value.hash_bytes
    }
}

impl Deref for Hash {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.hash_bytes
    }
}

impl Sha256Hash for Hash {
    fn hash_into_sha256(&self, hasher: &mut sha2::Sha256) {
        hasher.update(self.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::{Hash, Sha256Hash};

    #[test]
    fn hashes_composite_values_deterministically() {
        let value = (
            "alpha".to_owned(),
            vec![1u32, 2, 3],
            Some(("beta".to_owned(), false)),
        );

        let first = value.get_sha256();
        let second = value.get_sha256();

        assert_eq!(first, second);
        assert_eq!(first.to_hex().len(), 64);
    }

    #[test]
    fn hashes_hash_map_independently_of_insertion_order() {
        let mut left = HashMap::new();
        left.insert("alpha".to_owned(), 1u32);
        left.insert("beta".to_owned(), 2u32);

        let mut right = HashMap::new();
        right.insert("beta".to_owned(), 2u32);
        right.insert("alpha".to_owned(), 1u32);

        assert_eq!(left.get_sha256(), right.get_sha256());
    }

    #[test]
    fn hashes_hash_set_independently_of_insertion_order() {
        let mut left = HashSet::new();
        left.insert("alpha".to_owned());
        left.insert("beta".to_owned());

        let mut right = HashSet::new();
        right.insert("beta".to_owned());
        right.insert("alpha".to_owned());

        assert_eq!(left.get_sha256(), right.get_sha256());
    }

    #[test]
    fn normalizes_floating_point_edge_cases() {
        assert_eq!(0.0f32.get_sha256(), (-0.0f32).get_sha256());
        assert_eq!(0.0f64.get_sha256(), (-0.0f64).get_sha256());
        assert_eq!(f32::NAN.get_sha256(), f32::from_bits(0x7FC0_0001).get_sha256());
        assert_eq!(f64::NAN.get_sha256(), f64::from_bits(0x7FF8_0000_0000_0001).get_sha256());
    }

    #[test]
    fn hash_round_trips_bytes_and_formats_hex() {
        let bytes = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff, 0x10, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xa9, 0xba, 0xcb,
            0xdc, 0xed, 0xfe, 0x0f,
        ];
        let hash = Hash::from(bytes);

        assert_eq!(hash.as_bytes(), &bytes);
        assert_eq!(<[u8; 32]>::from(hash), bytes);
        assert_eq!(
            hash.to_hex().as_str(),
            "00112233445566778899aabbccddeeff102132435465768798a9bacbdcedfe0f"
        );
    }

    #[test]
    fn distinguishes_values_with_different_structure() {
        let tuple_hash = ("ab".to_owned(), "c".to_owned()).get_sha256();
        let split_tuple_hash = ("a".to_owned(), "bc".to_owned()).get_sha256();
        let vec_hash = vec!["ab".to_owned()].get_sha256();
        let array_hash = ["ab".to_owned()].get_sha256();

        assert_ne!(tuple_hash, split_tuple_hash);
        assert_ne!(vec_hash, array_hash);
    }
}
