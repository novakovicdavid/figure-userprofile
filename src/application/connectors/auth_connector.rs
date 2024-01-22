use async_trait::async_trait;

#[async_trait]
pub trait AuthConnector {
    async fn create_session(&self, user_id: i64, profile_id: i64) -> Result<String, AuthConnectorError>;
}

#[derive(Debug)]
pub enum AuthConnectorError {
    UnexpectedError(anyhow::Error),

    InvalidUserId,
    InvalidProfileId,
}

#[cfg(test)]
mod mock {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use uuid::Uuid;

    use crate::application::connectors::auth_connector::{AuthConnector, AuthConnectorError};

    pub struct MockAuthConnector(Arc<Mutex<Vec<(String, i64, i64)>>>);

    impl MockAuthConnector {
        pub fn new() -> Self {
            Self {
                0: Arc::new(Mutex::new(vec![])),
            }
        }
    }

    #[async_trait]
    impl AuthConnector for MockAuthConnector {
        async fn create_session(&self, user_id: i64, profile_id: i64) -> Result<String, AuthConnectorError> {
            let session_id = Uuid::new_v4().to_string();

            self.0.lock().unwrap()
                .push((session_id.clone(), user_id, profile_id));

            match (user_id, profile_id) {
                (user_id, _) if user_id < 1 => Err(AuthConnectorError::InvalidUserId),

                (_, profile_id) if profile_id < 1 => Err(AuthConnectorError::InvalidProfileId),

                (user_id, profile_id) => Ok(session_id),
            }
        }
    }
}