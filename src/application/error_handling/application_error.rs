pub use application_error::ApplicationError;

mod application_error {
    use axum::http::StatusCode;
    use axum::Json;
    use axum::response::IntoResponse;
    use axum_core::response::Response;
    use error_conversion_macro::ErrorEnum;
    use serde::Serialize;
    use thiserror::Error;
    use tracing::{error, Instrument, Span};

    use crate::application::error_handling::error_info::ErrorInfo;
    use crate::application::error_handling::RouteError;
    use crate::application::profile::service::ProfileServiceError;
    use crate::application::user_profile::service::UserProfileServiceError;

    #[derive(Debug, ErrorEnum, Error)]
    pub enum ApplicationError {
        #[error(transparent)]
        UnexpectedError(anyhow::Error),

        #[error(transparent)]
        UserProfileServiceError(UserProfileServiceError),

        #[error(transparent)]
        ProfileServiceError(ProfileServiceError),

        #[without_anyhow]
        #[error(transparent)]
        RouteError(RouteError),
    }

    impl ApplicationError {
        pub fn to_str(&self) -> String {
            match self {
                ApplicationError::UnexpectedError(_) => "internal-server-error".to_string(),

                ApplicationError::UserProfileServiceError(e) => e.to_string(),
                ApplicationError::ProfileServiceError(e) => e.to_string(),
                ApplicationError::RouteError(e) => e.to_string(),
            }
        }
    }

    #[derive(Serialize)]
    pub struct ErrorResponse<'a> {
        error: &'a str,
    }

    impl IntoResponse for ApplicationError {
        fn into_response(self) -> Response {
            let error_str = self.to_str();

            let status_code = match self {
                ApplicationError::UnexpectedError(error) => {
                    tokio::task::spawn(async move {
                        error!("Internal server error: {}\n{}", error, error.backtrace());
                    }.instrument(Span::current()));

                    500
                }

                ApplicationError::UserProfileServiceError(e) => e.status_code(),
                ApplicationError::ProfileServiceError(e) => e.status_code(),
                ApplicationError::RouteError(e) => e.status_code(),
            };

            (
                StatusCode::from_u16(status_code).unwrap(),
                Json(ErrorResponse {
                    error: &error_str
                })
            ).into_response()
        }
    }

    impl PartialEq for ApplicationError {
        fn eq(&self, other: &Self) -> bool {
            self.to_string() == other.to_string()
        }
    }
}