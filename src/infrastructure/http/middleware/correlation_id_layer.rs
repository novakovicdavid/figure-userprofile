use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::response::Response;
use axum_core::response::IntoResponse;
use http::{Request, StatusCode};
use tokio::task_local;
use tower::{Layer, Service};
use uuid::Uuid;

task_local! {
    static CORRELATION_ID: String;
}

pub fn get_correlation_id() -> Option<String> {
    CORRELATION_ID
        .try_with(|id| id.clone())
        .ok()
}

#[derive(Clone)]
pub struct CorrelationId(pub String);

#[derive(Clone)]
pub struct CorrelationLayer;

impl<S> Layer<S> for CorrelationLayer {
    type Service = CorrelationService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CorrelationService { inner }
    }
}

#[derive(Clone)]
pub struct CorrelationService<S> {
    inner: S,
}

impl<S, T> Service<Request<T>> for CorrelationService<S>
    where S: Service<Request<T>, Response=Response> + Send + 'static,
          S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<T>) -> Self::Future {
        let correlation_id = match get_or_extract_correlation_id(&request) {
            Ok(id) => id.unwrap_or_else(|| Uuid::new_v4().to_string()),

            Err(_) => return Box::pin(async move {
                Ok((StatusCode::BAD_REQUEST, "Could not parse x-correlation-id header.").into_response())
            })
        };

        request.extensions_mut().insert(CorrelationId(correlation_id.clone()));

        let future = self.inner.call(request);

        let closure = async move {
            let response: Response = future.await?;
            Ok(response)
        };

        let scoped_future = CORRELATION_ID.scope(correlation_id, closure);

        Box::pin(scoped_future)
    }
}

fn get_or_extract_correlation_id<T>(request: &Request<T>) -> Result<Option<String>, anyhow::Error> {
    get_correlation_id()
        .map_or_else(|| extract_correlation_id_from_header(request),
                     |id| Ok(Some(id)))
}

fn extract_correlation_id_from_header<T>(request: &Request<T>) -> Result<Option<String>, anyhow::Error> {
    request
        .headers()
        // Header may not exist, so the returned value here is Option<&HeaderValue>
        .get("x-correlation-id")

        // Try to parse the header value, this will become Option<Result<&Str...
        .map(|header| header
            .to_str()
            // Convert error to anyhow::Error
            .map_err(|e| e.into()))

        // Convert Option<Result<... into Result<Option<...
        .transpose()

        // Convert &str header value to String
        .map(|id| id.map(|id| id.to_string()))
}