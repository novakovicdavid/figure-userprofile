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

#[derive(Debug, Error)]
pub enum RouteError {
    #[error("invalid-multipart")]
    InvalidMultipart,
}