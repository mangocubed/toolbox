use std::fmt::Display;
use std::future::Future;

use cached::async_sync::OnceCell;
use cached::{AsyncRedisCache, IOCachedAsync};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::config::CACHE_CONFIG;

pub trait AsyncRedisCacheExt<K> {
    fn cache_remove(&self, prefix: &str, key: &K) -> impl Future<Output = ()> + Send;
}

impl<K, V> AsyncRedisCacheExt<K> for OnceCell<AsyncRedisCache<K, V>>
where
    K: Display + Send + Sync,
    V: DeserializeOwned + Display + Send + Serialize + Sync,
{
    async fn cache_remove(&self, prefix: &str, key: &K) {
        let _ = self
            .get_or_init(|| async { redis_cache_store(prefix).await })
            .await
            .cache_remove(key)
            .await;
    }
}

pub async fn redis_cache_store<K, V>(prefix: &str) -> AsyncRedisCache<K, V>
where
    K: Display + Send + Sync,
    V: DeserializeOwned + Display + Send + Serialize + Sync,
{
    AsyncRedisCache::new(format!("{prefix}:"), CACHE_CONFIG.ttl())
        .set_connection_string(&CACHE_CONFIG.redis_url)
        .set_refresh(true)
        .build()
        .await
        .expect("Could not get redis cache")
}
