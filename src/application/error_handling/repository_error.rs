pub use repository_error::RepositoryError;

mod repository_error {
    use thiserror::Error;
    use crate::application::error_handling::error_info::ErrorInfo;

    #[derive(Debug, Error)]
    pub enum RepositoryError {
        #[error(transparent)]
        UnexpectedError(anyhow::Error),

        #[error("resource-not-found")]
        ResourceNotFound,

        #[error("constraint-conflict")]
        ConstraintConflict,
    }

    impl ErrorInfo for RepositoryError {
        fn status_code(&self) -> u16 {
            match self {
                RepositoryError::UnexpectedError(_) => unreachable!(),
                RepositoryError::ResourceNotFound => 404,
                RepositoryError::ConstraintConflict => 409,
            }
        }
    }
}