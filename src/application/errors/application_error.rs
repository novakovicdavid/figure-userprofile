use error_conversion_macro::ErrorEnum;
use serde_json::Error;
use thiserror::Error;
use tracing::log::error;

use crate::application::errors::RouteError;
use crate::application::services::profile_service::ProfileServiceError;
use crate::application::services::user_service::UserProfileServiceError;

#[derive(Debug, ErrorEnum, Error)]
pub enum ApplicationError {
    #[error("internal-server-error")]
    UnexpectedError(anyhow::Error),

    #[error(transparent)]
    UserProfileServiceError(UserProfileServiceError),

    #[error(transparent)]
    ProfileServiceError(ProfileServiceError),

    #[without_anyhow]
    #[error(transparent)]
    RouteError(RouteError),
}

impl From<serde_json::Error> for ApplicationError {
    fn from(value: Error) -> Self {
        ApplicationError::UnexpectedError(value.into())
    }
}
