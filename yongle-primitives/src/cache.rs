/// Concurrent in-memory async cache.
pub type AsyncCache<K, V> = ::moka::future::Cache<K, V, ::ahash::RandomState>;
