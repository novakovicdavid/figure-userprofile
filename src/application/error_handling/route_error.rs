pub use route_error::RouteError;

mod route_error {
    use crate::application::error_handling::error_info::ErrorInfo;

    #[derive(Debug)]
    pub enum RouteError {
        InvalidMultipart,
    }

    impl ErrorInfo for RouteError {
        fn error_message(&self) -> &str {
            match self {
                RouteError::InvalidMultipart => "invalid-multipart"
            }
        }

        fn status_code(&self) -> u16 {
            match self {
                RouteError::InvalidMultipart => 400
            }
        }
    }
}