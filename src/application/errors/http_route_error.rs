use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouteError {
    #[error("invalid-multipart")]
    InvalidMultipart,
}