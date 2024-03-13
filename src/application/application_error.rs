pub use application_error::ApplicationError;

mod application_error {
    use error_conversion_macro::ErrorEnum;
    use thiserror::Error;
    use tracing::log::error;

    use crate::application::errors::RouteError;
    use crate::application::profile::service::ProfileServiceError;
    use crate::application::user_profile::service::UserProfileServiceError;

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
}