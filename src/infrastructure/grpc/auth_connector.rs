pub use auth_connector::GrpcAuthConnector;

mod auth_connector {
    use std::str::FromStr;

    use async_trait::async_trait;
    use tonic::transport::{Channel, Endpoint};
    use tower::ServiceBuilder;

    use auth_client::AuthClient;

    use crate::application::connectors::auth_connector::{AuthConnector, AuthConnectorError};

    tonic::include_proto!("auth");

    pub struct GrpcAuthConnector {
        client: AuthClient<Channel>,
    }

    impl GrpcAuthConnector {
        pub async fn connect(host: String, port: u16) -> Result<Self, anyhow::Error> {
            let endpoint = Endpoint::from_str(&format!("http://{host}:{port}"))?;

            let channel = endpoint.connect().await?;

            let channel = ServiceBuilder::new()
                .service(channel);

            Ok(Self {
                client: AuthClient::new(channel)
            })
        }
    }

    #[async_trait]
    impl AuthConnector for GrpcAuthConnector {
        async fn create_session(&self, user_id: String, profile_id: String) -> Result<String, AuthConnectorError> {
            let request = tonic::Request::new(CreateSessionRequest {
                user_id,
                profile_id,
            });

            self.client
                .clone()
                .create_session(request)
                .await
                .map(|response| response.into_inner().session_token)
                .map_err(|status| AuthConnectorError::UnexpectedError(anyhow::Error::msg(status)))
        }
    }
}