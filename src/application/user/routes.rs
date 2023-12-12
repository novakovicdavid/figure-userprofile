use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use cookie::{Cookie, SameSite};
use serde::Serialize;
use serde::Deserialize;
use tower_cookies::Cookies;
use derive_name::with_name;
use crate::application::server_errors::ServerError;
use crate::application::user::service::UserServiceTrait;
use crate::context::{ContextTrait, ServiceContextTrait};
use crate::infrastructure::session::{Session, SessionOption};
use crate::infrastructure::to_json_string::to_json_string_with_name;
use crate::ServerState;

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

pub async fn sign_in<C: ContextTrait>(Extension(_session_option): Extension<SessionOption>, State(server_state): State<Arc<ServerState<C>>>, cookies: Cookies, Json(signin): Json<SignInForm>) -> Response {
    return match server_state.context.service_context().user_service().sign_in(&signin.email, &signin.password).await {
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
        Err(e) => e.into_response()
    };
}

pub async fn sign_up<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, cookies: Cookies, Json(signup): Json<SignUpForm>) -> Response {
    return match server_state.context.service_context().user_service().sign_up(&signup.email, &signup.password, &signup.username).await {
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
        Err(e) => e.into_response()
    };
}

pub async fn sign_out<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, cookies: Cookies) -> Response {
    // if let Some(mut cookie) = cookies.get("session_id") {
    //     match server_state.context.repository_context().session_repository().remove_by_id(cookie.value()).await {
    //         Ok(_) => {
    //             cookie.set_http_only(true);
    //             cookie.set_secure(true);
    //             cookie.set_same_site(SameSite::Strict);
    //             cookie.set_domain(server_state.domain.to_string());
    //             cookie.set_path("/");
    //             cookie.make_removal();
    //             cookies.add(cookie.into_owned());
    //             StatusCode::OK.into_response()
    //         }
    //         Err(e) => e.into_response()
    //     }
    // } else {
    //     ServerError::NoSessionReceived.into_response()
    // }
    ServerError::NoSessionReceived.into_response()
}