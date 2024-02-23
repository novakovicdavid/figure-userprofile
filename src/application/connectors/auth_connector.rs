use async_trait::async_trait;
use thiserror::Error;

#[async_trait]
pub trait AuthConnector: Send + Sync {
    async fn create_session(&self, user_id: String, profile_id: String) -> Result<String, AuthConnectorError>;
}

#[derive(Debug, Error)]
pub enum AuthConnectorError {
    #[error(transparent)]
    UnexpectedError(anyhow::Error),
}

#[cfg(test)]
mod mock {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use uuid::Uuid;

    use crate::application::connectors::auth_connector::{AuthConnector, AuthConnectorError};

    pub struct MockAuthConnector(Arc<Mutex<Vec<(String, String, String)>>>);

    impl MockAuthConnector {
        pub fn new() -> Self {
            Self {
                0: Arc::new(Mutex::new(vec![])),
            }
        }
    }

    #[async_trait]
    impl AuthConnector for MockAuthConnector {
        async fn create_session(&self, user_id: String, profile_id: String) -> Result<String, AuthConnectorError> {
            let session_id = Uuid::new_v4().to_string();

            self.0.lock().unwrap()
                .push((session_id.clone(), user_id.clone(), profile_id.clone()));

            Ok(session_id)
        }
    }
}