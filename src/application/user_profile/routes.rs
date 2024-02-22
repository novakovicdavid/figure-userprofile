use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use cookie::{Cookie, SameSite};
use derive_name::with_name;
use serde::Deserialize;
use serde::Serialize;
use tower_cookies::Cookies;

use crate::application::ApplicationError;
use crate::application::transaction::TransactionTrait;
use crate::infrastructure::http::state::ServerState;
use crate::infrastructure::session::SessionOption;
use crate::infrastructure::to_json_string::to_json_string_with_name;

#[derive(Deserialize)]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct SignInForm {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[with_name(profile)]
struct SignInResponse {
    id: i64,
}

impl SignInResponse {
    pub fn new(id: i64) -> Self {
        Self {
            id,
        }
    }
}

pub async fn sign_in<T: TransactionTrait>(Extension(_session_option): Extension<SessionOption>, State(server_state): State<Arc<ServerState<T>>>, cookies: Cookies, Json(signin): Json<SignInForm>) -> impl IntoResponse {
    return match server_state.user_service.sign_in(&signin.email, &signin.password).await {
        Ok((profile_id, session_id)) => {
            let mut cookie = Cookie::new("session_id", session_id);
            cookie.set_http_only(true);
            cookie.set_secure(true);
            cookie.set_same_site(SameSite::Strict);
            cookie.set_domain(server_state.domain.to_string());
            cookie.set_path("/");
            cookies.add(cookie);
            to_json_string_with_name(SignInResponse::new(profile_id)).into_response()
        }
        Err(e) => ApplicationError::from(e).into_response()
    };
}

pub async fn sign_up<T: TransactionTrait>(State(server_state): State<Arc<ServerState<T>>>, cookies: Cookies, Json(signup): Json<SignUpForm>) -> Response {
    return match server_state.user_service.sign_up(&signup.email, &signup.password, &signup.username).await {
        Ok((profile_id, session)) => {
            let mut cookie = Cookie::new("session_id", session);
            cookie.set_http_only(true);
            cookie.set_secure(true);
            cookie.set_same_site(SameSite::Strict);
            cookie.set_domain(server_state.domain.to_string());
            cookie.set_path("/");
            cookies.add(cookie);
            to_json_string_with_name(SignInResponse::new(profile_id)).into_response()
        }
        Err(e) => ApplicationError::from(e).into_response()
    };
}
