use std::net::SocketAddr;

use axum::extract::connect_info::Connected;
use axum::serve::IncomingStream;

pub mod user_routes;
pub mod profile_routes;
mod error_response;

#[derive(Clone)]
pub struct ConnectionInfo {
    pub remote_addr: SocketAddr,
}

impl Connected<IncomingStream<'_>> for ConnectionInfo {
    fn connect_info(target: IncomingStream<'_>) -> Self {
        ConnectionInfo {
            remote_addr: target.remote_addr()
        }
    }
}