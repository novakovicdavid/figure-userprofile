use std::sync::Arc;

use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use cookie::{Cookie, SameSite};
use derive_name::with_name;
use serde::Deserialize;
use serde::Serialize;
use tower_cookies::Cookies;

use crate::application::ApplicationError;
use crate::application::miscellaneous::ToJsonString;
use crate::infrastructure::session::SessionOption;
use crate::state::ServerState;

pub fn user_router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/user/request-reset-password", post(reset_password))
        .route("/user/reset-password", post(reset_password))
        .route("/user/signup", post(sign_up))
        .route("/user/signin", post(sign_in))
}

#[derive(Serialize)]
#[with_name(profile)]
struct SignInResponse {
    pub id: String,
}

fn handle_sign_in_up(domain: String, cookies: &Cookies,
                     profile_id: String, session_id: String)
                     -> Result<String, ApplicationError>
{
    let cookie = create_session_cookie(domain, session_id);
    cookies.add(cookie);

    SignInResponse {
        id: profile_id,
    }.to_json_string()
}

#[derive(Deserialize)]
pub struct SignInForm {
    pub email: String,
    pub password: String,
}

pub async fn sign_in(Extension(_session_option): Extension<SessionOption>,
                     State(server_state): State<Arc<ServerState>>,
                     cookies: Cookies, Json(signin): Json<SignInForm>)
                     -> impl IntoResponse
{
    server_state.user_service.sign_in(&signin.email, &signin.password).await
        .map_err(ApplicationError::from)
        .and_then(|(profile_id, session)| handle_sign_in_up(server_state.domain.clone(), &cookies, profile_id, session))
}

#[derive(Deserialize)]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
    pub username: String,
}

pub async fn sign_up(State(server_state): State<Arc<ServerState>>,
                     cookies: Cookies, Json(signup): Json<SignUpForm>) -> impl IntoResponse
{
    server_state.user_service.sign_up(signup.email, signup.password, signup.username).await
        .map_err(ApplicationError::from)
        .and_then(|(profile_id, session)| handle_sign_in_up(server_state.domain.clone(), &cookies, profile_id, session))
}

fn create_session_cookie(domain: String, session: String) -> Cookie<'static> {
    let mut cookie = Cookie::new("session_id", session);
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_domain(domain);
    cookie.set_path("/");

    cookie
}

pub struct RequestResetPasswordRequest {
    pub email: String,
}

pub async fn request_reset_password(State(server_state): State<Arc<ServerState>>,
                                    Json(request): Json<RequestResetPasswordRequest>)
                                    -> impl IntoResponse
{
    todo!();
    // server_state.user_service.request_reset_password(email)
    //     .await
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}

pub async fn reset_password(State(server_state): State<Arc<ServerState>>,
                            Json(reset): Json<ResetPasswordRequest>)
                            -> impl IntoResponse
{
    server_state.user_service.reset_password(&reset.email, &reset.old_password, reset.new_password)
        .await
        .map_err(ApplicationError::from)
}
