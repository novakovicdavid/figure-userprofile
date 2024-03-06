use figure_lib::middleware::correlation_id::get_correlation_id;
use tonic::{Request, Status};
use tonic::service::Interceptor;

pub use auth_connector::GrpcAuthConnector;

mod auth_connector {
    use std::str::FromStr;

    use async_trait::async_trait;
    use tonic::codegen::InterceptedService;
    use tonic::transport::{Channel, Endpoint};
    use tower::ServiceBuilder;

    use auth_client::AuthClient;

    use crate::application::connectors::auth_connector::{AuthConnector, AuthConnectorError};
    use crate::infrastructure::grpc::auth_connector::CorrelationIdInterceptor;

    tonic::include_proto!("auth");

    pub struct GrpcAuthConnector {
        client: AuthClient<InterceptedService<Channel, CorrelationIdInterceptor>>,
    }

    impl GrpcAuthConnector {
        pub async fn connect(host: String, port: u16) -> Result<Self, anyhow::Error> {
            let endpoint = Endpoint::from_str(&format!("http://{host}:{port}"))?;

            let channel = endpoint.connect().await?;

            let channel = ServiceBuilder::new()
                .service(channel);

            let client = AuthClient::with_interceptor(channel, CorrelationIdInterceptor);

            Ok(Self {
                client
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

#[derive(Clone)]
pub struct CorrelationIdInterceptor;

impl Interceptor for CorrelationIdInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let correlation_id = get_correlation_id()
            .ok_or_else(|| Status::internal("Couldn't get the correlation id...?"))?;

        request
            .metadata_mut()
            .insert("x-correlation-id", correlation_id.parse().unwrap());

        Ok(request)
    }
}