use std::sync::Arc;

use error_conversion_macro::ErrorEnum;
use figure_lib::queue::integration::domain_event_dispatcher::DomainEventDispatcher;
use figure_lib::queue::internal_event_router::RouterError;
use figure_lib::rdbs::outbox_repository::{Outbox, OutboxError};
use figure_lib::rdbs::transaction::postgres_transaction::TransactionManager;
use figure_lib::rdbs::transaction::TransactionError;
use thiserror::Error;
use tracing::log::error;

use crate::application::connectors::auth_connector::{AuthConnector, AuthConnectorError};
use crate::application::domain_event_dispatcher::{DomainEvent, DomainEventDiscriminants};
use crate::application::errors::RepositoryError;
use crate::application::repository_traits::read::profile_repository::ProfileRepository;
use crate::application::repository_traits::read::user_repository::UserRepository;
use crate::application::state::DomainEventHandlerState;
use crate::domain::{Profile, User};
use crate::domain::profile::ProfileDomainError;
use crate::domain::user::UserDomainError;

pub struct UserProfileService {
    transaction_manager: TransactionManager,
    domain_event_dispatcher: Arc<DomainEventDispatcher
    <DomainEventDiscriminants, DomainEvent, Arc<DomainEventHandlerState>>>,
    user_repository: Box<dyn UserRepository>,
    profile_repository: Box<dyn ProfileRepository>,
    outbox_repository: Box<dyn Outbox>,
    auth_connector: Box<dyn AuthConnector>,
}

#[derive(Debug, ErrorEnum, Error)]
pub enum UserProfileServiceError {
    #[error("email-already-in-use")]
    EmailAlreadyInUse,

    #[error(transparent)]
    UserDomainError(UserDomainError),

    #[without_anyhow]
    #[error(transparent)]
    ProfileDomainError(ProfileDomainError),

    #[error(transparent)]
    RepositoryError(RepositoryError),
    #[error(transparent)]
    TransactionError(TransactionError),
    #[error(transparent)]
    AuthConnectorError(AuthConnectorError),
    #[error(transparent)]
    OutboxError(OutboxError),
    #[error(transparent)]
    RouterError(RouterError),

    #[error(transparent)]
    UnexpectedError(anyhow::Error),
}

impl UserProfileService {
    pub fn new(transaction_manager: TransactionManager,
               domain_event_dispatcher: Arc<DomainEventDispatcher<DomainEventDiscriminants, DomainEvent, Arc<DomainEventHandlerState>>>,
               user_repository: Box<dyn UserRepository>,
               profile_repository: Box<dyn ProfileRepository>,
               outbox_repository: Box<dyn Outbox>,
               auth_connector: Box<dyn AuthConnector>) -> Self {
        UserProfileService {
            user_repository,
            profile_repository,
            transaction_manager,
            auth_connector,
            outbox_repository,
            domain_event_dispatcher,
        }
    }

    pub async fn sign_up(&self, email: String, password: String, username: String) -> Result<(String, String), UserProfileServiceError> {
        User::validate_email(&email)?;
        User::validate_password(&password)?;
        Profile::validate_username(&username)?;

        if self.user_repository.find_one_by_email(&email).await.is_ok() {
            return Err(UserProfileServiceError::EmailAlreadyInUse);
        }

        let (user, profile) = User::register(email, password, username)?;

        let result: Result<_, UserProfileServiceError> = self.transaction_manager.transaction(|| async move {
            self.user_repository.insert(&user).await?;
            self.profile_repository.insert(&profile).await?;

            Ok((user, profile))
        }).await?;


        let (user, profile) = result?;

        let session_id = self.auth_connector
            .create_session(user.get_id(), profile.get_id())
            .await?;

        Ok((profile.get_id(), session_id))
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<(String, String), UserProfileServiceError> {
        User::validate_email(email)?;
        User::validate_password(password)?;

        let user = self.user_repository.find_one_by_email(email).await?;

        user.login(&password)?;

        let profile = self.profile_repository.find_by_user_id(user.get_id()).await?;

        let session_id = self.auth_connector
            .create_session(user.get_id(), profile.get_id())
            .await?;

        Ok((profile.get_id(), session_id))
    }

    pub async fn request_reset_password(&self, email: &str, requester: String) -> Result<(), UserProfileServiceError> {
        let token = self.transaction_manager.transaction(|| async {
            let mut user = self.user_repository.find_one_by_email(&email).await?;

            let event = user.request_password_reset(requester)?;

            self.user_repository.update(&user).await?;

            self.domain_event_dispatcher.dispatch(event).await?;

            Ok::<_, UserProfileServiceError>(())
        }).await??;

        // todo send email with token

        Ok(())
    }

    pub async fn reset_password(&self, token: &str, new_password: &str) -> Result<(), UserProfileServiceError> {
        User::validate_password(&new_password)?;

        let mut user = self.user_repository.find_by_reset_password_token(&token).await?;

        let event = user.reset_password_using_password_reset_token(&token, &new_password)?;

        self.transaction_manager.transaction(|| async {
            self.user_repository.update(&user).await?;

            // todo this shit
            // self.outbox_repository.insert(event.into()).await?;
            Ok::<(), UserProfileServiceError>(())
        }).await??;

        Ok(())
    }
}