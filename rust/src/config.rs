use std::sync::LazyLock;

use envconfig::Envconfig;
use url::Url;

pub(crate) static IDENTITY_CONFIG: LazyLock<IdentityConfig> =
    LazyLock::new(|| IdentityConfig::init_from_env().unwrap());
pub(crate) static SENTRY_CONFIG: LazyLock<SentryConfig> = LazyLock::new(|| SentryConfig::init_from_env().unwrap());

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
