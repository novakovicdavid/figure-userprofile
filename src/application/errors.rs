use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error(transparent)]
    UnexpectedError(anyhow::Error),

    #[error("resource-not-found")]
    ResourceNotFound,

    #[error("constraint-conflict")]
    ConstraintConflict,
}

impl From<tokio_postgres::Error> for RepositoryError {
    fn from(value: tokio_postgres::Error) -> Self {
        if let Some(db_error) = value.as_db_error() {
            if let Some(_constraint) = db_error.constraint() {
                return RepositoryError::ConstraintConflict
            }
        }

        RepositoryError::UnexpectedError(value.into())
    }
}

impl From<deadpool_postgres::PoolError> for RepositoryError {
    fn from(value: deadpool_postgres::PoolError) -> Self {
        RepositoryError::UnexpectedError(value.into())
    }
}

#[derive(Debug, Error)]
pub enum RouteError {
    #[error("invalid-multipart")]
    InvalidMultipart,
}