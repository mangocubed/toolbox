use async_graphql::{ID, Result};
use uuid::Uuid;

pub trait IDExt {
    fn try_into_uuid(&self) -> Result<Uuid>;
}

impl IDExt for ID {
    fn try_into_uuid(&self) -> Result<Uuid> {
        Ok(Uuid::try_parse(self.as_ref())?)
    }
}

#[cfg(feature = "identity-client")]
pub mod objects {
    use async_graphql::{ID, Object};
    use chrono::{DateTime, Utc};
    use url::Url;

    use crate::identity_client::IdentityUser;

    pub struct IdentityUserObject<'a>(pub IdentityUser<'a>);

    #[Object]
    impl IdentityUserObject<'_> {
        async fn id(&self) -> ID {
            self.0.id.into()
        }

        async fn username(&self) -> &str {
            &self.0.username
        }

        async fn display_name(&self) -> &str {
            &self.0.display_name
        }

        async fn initials(&self) -> &str {
            &self.0.initials
        }

        async fn language_code(&self) -> &str {
            &self.0.language_code
        }

        async fn country_code(&self) -> &str {
            &self.0.country_code
        }

        async fn avatar_image_url(&self) -> Url {
            self.0.avatar_image_url.clone()
        }

        async fn created_at(&self) -> DateTime<Utc> {
            self.0.created_at
        }

        async fn updated_at(&self) -> Option<DateTime<Utc>> {
            self.0.updated_at
        }
    }
}
