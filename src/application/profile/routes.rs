use std::sync::Arc;
use anyhow::Context;
use axum::Extension;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use tower_cookies::Cookies;
use crate::application::profile::dtos::{ProfileDTO, ProfileWithoutUserIdDTO};
use crate::application::profile::service::ProfileServiceTrait;
use crate::context::{ContextTrait, ServiceContextTrait};
use crate::infrastructure::session::SessionOption;
use crate::infrastructure::to_json_string::to_json_string_with_name;
use crate::server_errors::ServerError;
use crate::ServerState;

pub async fn get_profile<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, Path(profile_id): Path<i64>) -> impl IntoResponse {
    server_state.context.service_context().profile_service()
        .find_profile_by_id(profile_id)
        .await
        .and_then(|profile| Ok(ProfileWithoutUserIdDTO::from(profile)))
        .map(to_json_string_with_name)
}

pub async fn get_total_profiles_count<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>) -> impl IntoResponse {
    server_state.context.service_context().profile_service()
        .get_total_profiles_count()
        .await
        .map(|count| count.to_string())
}

pub async fn update_profile<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, session: Extension<SessionOption>, multipart: Multipart) -> impl IntoResponse {
    // Check if logged in
    let session = match &session.session {
        Some(s) => s,
        None => return StatusCode::UNAUTHORIZED.into_response()
    };

    // Parse multipart
    let result = parse_update_profile_multipart(multipart).await;
    let (display_name, bio) = match result {
        Ok(tuple) => tuple,
        Err(_) => return ServerError::InvalidMultipart.into_response()
    };

    // Update profile
    server_state.context.service_context().profile_service()
        .update_profile_by_id(session.profile_id, display_name, bio)
        .await
        .into_response()
}

async fn parse_update_profile_multipart(mut multipart: Multipart) -> Result<(Option<String>, Option<String>), anyhow::Error> {
    let mut display_name: Option<String> = None;
    let mut bio: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().context("Multipart parse failed: no field name")?.to_string();
        let data = field.bytes().await?;

        match name.as_str() {
            "display_name" => display_name = Some(String::from_utf8(data.to_vec())?),
            "bio" => bio = Some(String::from_utf8(data.to_vec())?),
            _ => {}
        };
    };

    Ok((display_name, bio))
}

// Return the profile associated with a given session
pub async fn load_profile_from_session<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, cookies: Cookies) -> Response {
    // if let Some(cookie) = cookies.get("session_id") {
    //     match server_state.context.repository_context().session_repository().find_by_id(cookie.value(), Some(86400)).await {
    //         Ok(session_data) => {
    //             server_state.context.service_context().profile_service().find_profile_by_id(session_data.get_profile_id())
    //                 .await
    //                 .map(ProfileDTO::from)
    //                 .map(to_json_string_with_name)
    //                 .into_response()
    //         }
    //         Err(e) => e.into_response()
    //     }
    // } else {
    //     ServerError::NoSessionReceived.into_response()
    // }
    ServerError::NoSessionReceived.into_response()
}