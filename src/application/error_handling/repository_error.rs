pub use repository_error::RepositoryError;

mod repository_error {
    use crate::application::error_handling::error_info::ErrorInfo;

    #[derive(Debug)]
    pub enum RepositoryError {
        UnexpectedError(anyhow::Error),
        ResourceNotFound,
        ConstraintConflict,
    }

    impl ErrorInfo for RepositoryError {
        fn error_message(&self) -> &str {
            match self {
                RepositoryError::UnexpectedError(_) => unreachable!(),
                RepositoryError::ResourceNotFound => "resource-not-found",
                RepositoryError::ConstraintConflict => "constraint-conflict"
            }
        }

        fn status_code(&self) -> u16 {
            match self {
                RepositoryError::UnexpectedError(_) => unreachable!(),
                RepositoryError::ResourceNotFound => 404,
                RepositoryError::ConstraintConflict => 409,
            }
        }
    }
}