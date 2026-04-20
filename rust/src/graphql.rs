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
