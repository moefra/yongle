/// The papaya-based thread-safe map implementation.
///
/// The result of iteration of this map is not ordered.
///
/// Please do not rely on any specific order.
pub type ConcurrentMap<K, V> = ::papaya::HashMap<K, V, ::ahash::RandomState>;

/// The papaya-based thread-safe set implementation.
///
/// The result of iteration of this set is not ordered.
///
/// Please do not rely on any specific order.
pub type ConcurrentSet<K> = ::papaya::HashSet<K, ::ahash::RandomState>;

/// Anti-hash DoS attack resistant map.
pub type SafeMap<K, V> =
    ::std::collections::HashMap<K, V, ::std::collections::hash_map::RandomState>;

/// Anti-hash DoS attack resistant set.
pub type SafeSet<V> = ::std::collections::HashSet<V, ::std::collections::hash_map::RandomState>;

/// A fast hash map.
///
/// Not thread-safe.
///
/// Iteration order is unspecified.
///
/// Please do not rely on any specific order.
///
/// Uses a fast non-cryptographic hasher suitable for in-memory hash maps.
pub type FastMap<K, V> = ::std::collections::HashMap<K, V, ::ahash::RandomState>;

/// A fast hash set.
///
/// Not thread-safe.
///
/// Iteration order is unspecified.
///
/// Please do not rely on any specific order.
///
/// Uses a fast non-cryptographic hasher suitable for in-memory hash maps.
pub type FastSet<K> = ::std::collections::HashSet<K, ::ahash::RandomState>;

/// A fast integer-keyed hash map.
///
/// Intended for integer keys or newtypes explicitly enabled for
/// `nohash_hasher::IsEnabled`.
///
/// Not thread-safe.
///
/// Iteration order is unspecified.
///
/// Please do not rely on any specific order.
pub type IntMap<K, V> = ::std::collections::HashMap<K, V, ::nohash_hasher::BuildNoHashHasher<K>>;

/// A fast integer-keyed hash set.
///
/// Intended for integer keys or newtypes explicitly enabled for
/// `nohash_hasher::IsEnabled`.
///
/// Not thread-safe.
///
/// Iteration order is unspecified.
///
/// Please do not rely on any specific order.
pub type IntSet<K> = ::std::collections::HashSet<K, ::nohash_hasher::BuildNoHashHasher<K>>;

/// Iteration order is deterministic and sorted by key.
pub type SortedMap<K, V> = ::std::collections::BTreeMap<K, V>;

/// Iteration order is deterministic and sorted by key.
pub type SortedSet<K> = ::std::collections::BTreeSet<K>;
