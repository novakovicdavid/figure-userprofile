use std::env;
use std::env::VarError;

use tracing::log::{error, warn};

use crate::application::errors::ApplicationError;

#[derive(Clone)]
pub struct Environment {
    pub database_url: String,

    // CORS origin
    pub origin: String,

    pub server_port: u16,

    pub auth_host: String,
    pub auth_port: u16,
}

impl Environment {
    pub fn new() -> Result<Self, ApplicationError> {
        Ok(
            Self {
                database_url: get_var("DATABASE_URL").expect("No DATABASE_URL env found"),
                origin: get_var("ORIGIN").expect("No ORIGIN env found"),
                server_port: get_var("SERVER_PORT").unwrap_or_else(|e| {
                    let error_reason = match e {
                        VarError::NotPresent => "Environment variable SERVER_PORT not found",
                        VarError::NotUnicode(e) => {
                            error!("{:#?}", e);
                            "Environment variable invalid"
                        }
                    };

                    warn!("{error_reason}, defaulting to port 8000");
                    "8000".to_string()
                }).parse::<u16>().expect("Invalid SERVER_PORT env"),
                auth_host: get_var("AUTH_HOST").expect("No AUTH_HOST env found"),
                auth_port: get_var("AUTH_PORT").expect("No AUTH_PORT env found").parse().unwrap(),
            }
        )
    }
}

fn get_var(key: &str) -> Result<String, VarError> {
    let result = env::var(key);
    env::remove_var(key);
    result
}