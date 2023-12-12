use std::env;
use std::env::VarError;
use tracing::{error, warn};
use crate::server_errors::ServerError;

pub struct Environment {
    pub database_url: String,

    // CORS origin
    pub origin: String,

    pub server_port: u16,

    // Loki logging server url & name of running figure-backend instance
    pub loki_host: Option<String>,
    pub loki_url: Option<String>,
}

impl Environment {
    pub fn new() -> Result<Self, ServerError> {
        Ok(
            Self {
                database_url: Self::get_var("DATABASE_URL").expect("No DATABASE_URL env found"),
                origin: Self::get_var("ORIGIN").expect("No ORIGIN env found"),
                loki_host: Self::get_var("LOKI_HOST").ok(),
                loki_url: Self::get_var("LOKI_URL").ok(),
                server_port: Self::get_var("SERVER_PORT").unwrap_or_else(|e| {
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
            }
        )
    }

    fn get_var(key: &str) -> Result<String, VarError> {
        let result = env::var(key);
        env::remove_var(key);
        result
    }
}