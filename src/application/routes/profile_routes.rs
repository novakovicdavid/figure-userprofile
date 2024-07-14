use std::sync::Arc;

use axum::{Extension, Router};
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde::Serialize;
use tower_http::limit::RequestBodyLimitLayer;

use crate::application::errors::ApplicationError;
use crate::application::errors::RouteError;
use crate::application::miscellaneous::ToJsonString;
use crate::application::state::ServerState;
use crate::infrastructure::session::SessionOption;

pub fn profile_router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/profile/update", post(update_profile)
            // Set a different limit
            .layer(RequestBodyLimitLayer::new(5 * 1_000_000)))

        .route("/profiles/:id", get(get_profile))
        .route("/profiles/count", get(get_total_profiles_count))
}

#[derive(Serialize, Debug)]
pub struct GetProfileResponseDTO {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub banner: Option<String>,
    pub profile_picture: Option<String>,
}

pub async fn get_profile(State(server_state): State<Arc<ServerState>>, Path(profile_id): Path<String>) -> impl IntoResponse {
    server_state.profile_service
        .find_profile_by_id(profile_id).await
        .map(|profile| GetProfileResponseDTO {
            id: profile.id,
            username: profile.username,
            display_name: profile.display_name,
            bio: profile.bio,
            banner: profile.banner,
            profile_picture: profile.profile_picture,
        })
        .map_err(ApplicationError::from)
        .and_then(|dto| dto.to_json_string())
}

pub async fn get_total_profiles_count(State(server_state): State<Arc<ServerState>>) -> impl IntoResponse {
    server_state.profile_service
        .get_total_profiles_count()
        .await
        .map(|count| count.to_string())
        .map_err(ApplicationError::from)
}

pub async fn update_profile(State(server_state): State<Arc<ServerState>>, session: Extension<SessionOption>, multipart: Multipart) -> impl IntoResponse {
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
        .update_profile_by_id(session.profile_id.clone(), display_name, bio)
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