use tokio_postgres::Row;

use crate::application::errors::RepositoryError;
use crate::domain::user::user::ResetPasswordRequest;

pub struct ResetPasswordRequestEntity {
    user_id: String,
    token: String,
    datetime: i64,
}

impl TryFrom<Row> for ResetPasswordRequestEntity {
    type Error = RepositoryError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let user_id = value.try_get("user_id")?;
        let token = value.try_get("token")?;
        let datetime = value.try_get("datetime")?;

        Ok(Self {
            user_id,
            token,
            datetime,
        })
    }
}

impl From<ResetPasswordRequestEntity> for ResetPasswordRequest {
    fn from(value: ResetPasswordRequestEntity) -> Self {
        Self {
            token: value.token,
            datetime: value.datetime,
        }
    }
}