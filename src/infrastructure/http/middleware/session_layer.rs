use axum::middleware::Next;
use axum_core::extract::Request;
use axum_core::response::{IntoResponse, Response};
use http::{HeaderValue, StatusCode};

use crate::infrastructure::session::{Session, SessionOption};

pub async fn session_extension(mut req: Request, next: Next) -> Response {
    let headers = req.headers();

    let user_id = headers.get("user_id");
    let profile_id = headers.get("profile_id");

    let session = handle_session_headers(user_id, profile_id);

    if session.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let session = session.unwrap();

    req
        .extensions_mut()
        .insert(session);

    let mut response = next.run(req).await;

    let response_headers = response.headers_mut();

    response_headers.remove("user_id");
    response_headers.remove("profile_id");

    response
}

fn handle_session_headers(user_id: Option<&HeaderValue>, profile_id: Option<&HeaderValue>) -> Result<SessionOption, anyhow::Error> {
    match (user_id, profile_id) {
        (Some(user_id), Some(profile_id)) => {
            Ok(SessionOption::from(Session::new(user_id.to_str()?.to_string(),
                                                profile_id.to_str()?.to_string())))
        }

        _ => Ok(SessionOption::new())
    }
}