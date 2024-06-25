use error_conversion_macro::ErrorEnum;
use figure_lib::rdbs::outbox_repository::{Outbox, OutboxError};
use figure_lib::rdbs::transaction::postgres_transaction::TransactionManager;
use figure_lib::rdbs::transaction::TransactionError;
use thiserror::Error;
use tracing::log::error;

use crate::application::connectors::auth_connector::{AuthConnector, AuthConnectorError};
use crate::application::errors::RepositoryError;
use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::application::user_profile::repository::UserRepositoryTrait;
use crate::domain::profile::ProfileDomainError;
use crate::domain::User;
use crate::domain::user::UserDomainError;

pub struct UserProfileService {
    transaction_manager: TransactionManager,
    user_repository: Box<dyn UserRepositoryTrait>,
    profile_repository: Box<dyn ProfileRepositoryTrait>,
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
    UnexpectedError(anyhow::Error),
}

impl UserProfileService {
    pub fn new(transaction_manager: TransactionManager,
               user_repository: Box<dyn UserRepositoryTrait>,
               profile_repository: Box<dyn ProfileRepositoryTrait>,
               outbox_repository: Box<dyn Outbox>,
               auth_connector: Box<dyn AuthConnector>) -> Self {
        UserProfileService {
            user_repository,
            profile_repository,
            transaction_manager,
            auth_connector,
            outbox_repository,
        }
    }

    pub async fn sign_up(&self, email: String, password: String, username: String) -> Result<(String, String), UserProfileServiceError> {
        User::validate_email(&email)?;
        User::validate_password(&password)?;

        if self.user_repository.find_one_by_email(&email).await.is_ok() {
            return Err(UserProfileServiceError::EmailAlreadyInUse);
        }

        let (user, profile) = User::register(email, password, username)?;

        let result: Result<_, UserProfileServiceError> = self.transaction_manager.transaction(|| async move {
            self.user_repository.insert(&user).await?;
            self.profile_repository.create(&profile).await?;

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

    pub async fn reset_password(&self, email: &str, old_password: &str, new_password: String) -> Result<(), UserProfileServiceError> {
        User::validate_password(&new_password)?;

        let mut user = self.user_repository.find_one_by_email(email).await?;

        let event = user.reset_password(old_password, &new_password)?;

        self.transaction_manager.transaction(|| async {
            self.user_repository.set_password(&user).await?;
            self.outbox_repository.insert(event.into()).await?;
            Ok::<(), UserProfileServiceError>(())
        }).await??;

        Ok(())
    }
}