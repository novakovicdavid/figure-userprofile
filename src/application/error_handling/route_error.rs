pub use route_error::RouteError;

mod route_error {
    use thiserror::Error;

    use crate::application::error_handling::error_info::ErrorInfo;

    #[derive(Debug, Error)]
    pub enum RouteError {
        #[error("invalid-multipart")]
        InvalidMultipart,
    }

    impl ErrorInfo for RouteError {
        fn status_code(&self) -> u16 {
            match self {
                RouteError::InvalidMultipart => 400
            }
        }
    }
}