use time::{OffsetDateTime, PrimitiveDateTime};
use tokio_postgres::Row;

use crate::application::errors::RepositoryError;
use crate::domain::user::user::ResetPasswordRequest;

pub struct ResetPasswordRequestEntity {
    token: String,
    user_id: String,
    datetime: OffsetDateTime,
}

impl TryFrom<Row> for ResetPasswordRequestEntity {
    type Error = RepositoryError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let token = value.try_get("token")?;
        let user_id = value.try_get("user_id")?;
        let datetime = value.try_get::<_, PrimitiveDateTime>("datetime")?
            .assume_utc();

        Ok(Self {
            token,
            user_id,
            datetime,
        })
    }
}

impl From<ResetPasswordRequestEntity> for ResetPasswordRequest {
    fn from(value: ResetPasswordRequestEntity) -> Self {
        Self::new(value.token, value.datetime)
    }
}