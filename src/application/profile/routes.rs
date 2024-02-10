use std::sync::Arc;

use axum::Extension;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::application::ApplicationError;
use crate::application::error_handling::RouteError;
use crate::application::profile::dtos::ProfileWithoutUserIdDTO;
use crate::application::transaction::TransactionTrait;
use crate::infrastructure::session::SessionOption;
use crate::infrastructure::to_json_string::to_json_string_with_name;
use crate::ServerState;

pub async fn get_profile<T: TransactionTrait>(State(server_state): State<Arc<ServerState<T>>>, Path(profile_id): Path<i64>) -> impl IntoResponse {
    server_state.profile_service
        .find_profile_by_id(profile_id)
        .await
        .and_then(|profile| Ok(ProfileWithoutUserIdDTO::from(profile)))
        .map(to_json_string_with_name)
        .map_err(ApplicationError::from)
}

pub async fn get_total_profiles_count<T: TransactionTrait>(State(server_state): State<Arc<ServerState<T>>>) -> impl IntoResponse {
    server_state.profile_service
        .get_total_profiles_count()
        .await
        .map(|count| count.to_string())
        .map_err(ApplicationError::from)
}

pub async fn update_profile<T: TransactionTrait>(State(server_state): State<Arc<ServerState<T>>>, session: Extension<SessionOption>, multipart: Multipart) -> impl IntoResponse {
    // Check if logged in
    let session = match &session.session {
        Some(s) => s,
        None => return StatusCode::UNAUTHORIZED.into_response()
    };

    // Parse multipart
    let multipart_result = parse_update_profile_multipart(multipart).await;

    let (display_name, bio) = match multipart_result {
        Some(tuple) => tuple,
        None => return ApplicationError::from(RouteError::InvalidMultipart).into_response()
    };

    // Update profile
    server_state.profile_service
        .update_profile_by_id(session.profile_id, display_name, bio)
        .await
        .map_err(ApplicationError::from)
        .into_response()
}

async fn parse_update_profile_multipart(mut multipart: Multipart) -> Option<(Option<String>, Option<String>)> {
    let mut display_name: Option<String> = None;
    let mut bio: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name()?.to_string();
        let data = field.bytes().await.ok()?;

        match name.as_str() {
            "display_name" => display_name = Some(String::from_utf8(data.to_vec()).ok()?),
            "bio" => bio = Some(String::from_utf8(data.to_vec()).ok()?),
            _ => {}
        };
    };

    Some((display_name, bio))
}