pub use application_error::ApplicationError;

mod application_error {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    use axum::http::StatusCode;
    use axum::Json;
    use axum::response::IntoResponse;
    use axum_core::response::Response;
    use serde::Serialize;
    use tracing::{error, Instrument, Span};
    use crate::application::error_handling::error_info::ErrorInfo;

    use crate::application::error_handling::RouteError;
    use crate::application::profile::service::ProfileServiceError;
    use crate::application::user::service::UserServiceError;

    #[derive(Debug)]
    pub enum ApplicationError {
        UnexpectedError(anyhow::Error),

        UserServiceError(UserServiceError),
        ProfileServiceError(ProfileServiceError),

        RouteError(RouteError),
    }

    impl ApplicationError {
        pub fn to_str(&self) -> &str {
            match self {
                ApplicationError::UnexpectedError(_) => "internal-server-error",

                ApplicationError::UserServiceError(e) => e.error_message(),
                ApplicationError::ProfileServiceError(e) => e.error_message(),
                ApplicationError::RouteError(e) => e.error_message(),
            }
        }
    }

    impl Error for ApplicationError {}

    #[derive(Serialize)]
    pub struct ErrorResponse<'a> {
        error: &'a str,
    }

    impl Display for ApplicationError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_str())
        }
    }

    impl IntoResponse for ApplicationError {
        fn into_response(self) -> Response {
            let error_str = self.to_string();

            let status_code = match self {
                ApplicationError::UnexpectedError(error) => {
                    tokio::task::spawn(async move {
                        error!("Internal server error: {}\n{}", error, error.backtrace());
                    }.instrument(Span::current()));

                    500
                }

                ApplicationError::UserServiceError(e) => e.status_code(),
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

    impl From<RouteError> for ApplicationError {
        fn from(value: RouteError) -> Self {
            ApplicationError::RouteError(value)
        }
    }

    impl From<UserServiceError> for ApplicationError {
        fn from(value: UserServiceError) -> Self {
            match value {
                UserServiceError::UnexpectedError(e) => ApplicationError::UnexpectedError(e),
                _ => ApplicationError::UserServiceError(value),
            }
        }
    }

    impl From<ProfileServiceError> for ApplicationError {
        fn from(value: ProfileServiceError) -> Self {
            match value {
                ProfileServiceError::UnexpectedError(e) => ApplicationError::UnexpectedError(e),
                _ => ApplicationError::ProfileServiceError(value),
            }
        }
    }
}