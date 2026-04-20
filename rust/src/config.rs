use std::sync::LazyLock;
use std::time::Duration;

use envconfig::Envconfig;
use url::Url;

pub(crate) static CACHE_CONFIG: LazyLock<CacheConfig> = LazyLock::new(|| CacheConfig::init_from_env().unwrap());
pub(crate) static IDENTITY_CONFIG: LazyLock<IdentityConfig> =
    LazyLock::new(|| IdentityConfig::init_from_env().unwrap());
pub(crate) static SENTRY_CONFIG: LazyLock<SentryConfig> = LazyLock::new(|| SentryConfig::init_from_env().unwrap());

#[derive(Envconfig)]
pub struct CacheConfig {
    #[envconfig(from = "CACHE_REDIS_URL", default = "redis://127.0.0.1:6379/0")]
    pub redis_url: String,
    #[envconfig(from = "CACHE_TTL_SECS", default = "3600")]
    ttl_secs: u16,
}

impl CacheConfig {
    pub fn ttl(&self) -> Duration {
        Duration::from_secs(self.ttl_secs as u64)
    }
}

#[derive(Envconfig)]
pub(crate) struct IdentityConfig {
    #[envconfig(from = "IDENTITY_API_URL", default = "https://api.id.mango3.app/")]
    pub(crate) api_url: Url,
}

#[derive(Envconfig)]
pub struct SentryConfig {
    #[envconfig(from = "SENTRY_DSN")]
    pub dsn: Option<String>,
    #[envconfig(from = "SENTRY_TRACES_SAMPLE_RATE", default = "1.0")]
    pub traces_sample_rate: f32,
    #[envconfig(from = "SENTRY_SEND_DEFAULT_PII", default = "true")]
    pub send_default_pii: bool,
}
