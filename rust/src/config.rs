use std::sync::LazyLock;

use envconfig::Envconfig;
use url::Url;

pub(crate) static IDENTITY_CONFIG: LazyLock<IdentityConfig> =
    LazyLock::new(|| IdentityConfig::init_from_env().unwrap());

#[derive(Envconfig)]
pub(crate) struct IdentityConfig {
    #[envconfig(from = "IDENTITY_API_URL", default = "https://api.id.mango3.app/")]
    pub(crate) api_url: Url,
}
