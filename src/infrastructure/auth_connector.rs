mod auth_connector {
    use async_trait::async_trait;

    use auth_client::AuthClient;

    use crate::application::connectors::auth_connector::{AuthConnector, AuthConnectorError};

    tonic::include_proto!("auth");

    pub struct GrpcAuthConnector {
        host: String,
        port: u16,
    }

    #[async_trait]
    impl AuthConnector for GrpcAuthConnector {
        async fn create_session(&self, user_id: i64, profile_id: i64) -> Result<String, AuthConnectorError> {
            let mut client = AuthClient
            ::connect(format!("https://{}:{}", self.host, self.port))
                .await
                .map_err(|e| AuthConnectorError::UnexpectedError(e.into()))?;

            let request = tonic::Request::new(CreateSessionRequest {
                user_id,
                profile_id,
            });

            client
                .create_session(request)
                .await
                .map(|response| response.into_inner().session_token)
                .map_err(|status| match status.message() {
                    "invalid-user-id" => AuthConnectorError::InvalidUserId,
                    "invalid-profile-id" => AuthConnectorError::InvalidProfileId,
                    _ => AuthConnectorError::UnexpectedError(anyhow::Error::msg(status))
                })
        }
    }
}