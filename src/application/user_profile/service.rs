use std::marker::PhantomData;

use error_conversion_macro::ErrorEnum;
use figure_lib::rdbs::transaction::{TransactionError, TransactionManagerTrait, TransactionTrait};
use thiserror::Error;
use tracing::log::error;
use uuid::Uuid;

use crate::application::connectors::auth_connector::{AuthConnector, AuthConnectorError};
use crate::application::errors::RepositoryError;
use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::application::user_profile::repository::UserRepositoryTrait;
use crate::domain::{Profile, User};
use crate::domain::profile::ProfileDomainError;
use crate::domain::user::UserDomainError;
use crate::infrastructure::secure_hasher::{SecureHasher, SecureHasherError};

pub struct UserProfileService<T> {
    transaction_creator: Box<dyn TransactionManagerTrait<T>>,
    marker: PhantomData<T>,
    user_repository: Box<dyn UserRepositoryTrait<T>>,
    profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
    secure_hasher: Box<dyn SecureHasher>,
    auth_connector: Box<dyn AuthConnector>,
}

#[derive(Debug, ErrorEnum, Error)]
pub enum UserProfileServiceError {
    #[error("email-already-in-use")]
    EmailAlreadyInUse,

    #[without_anyhow]
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
    SecureHasherError(SecureHasherError),
    #[error(transparent)]
    AuthConnectorError(AuthConnectorError),

    #[error(transparent)]
    UnexpectedError(anyhow::Error),
}

impl<T> UserProfileService<T> where T: TransactionTrait
{
    pub fn new(transaction_creator: Box<dyn TransactionManagerTrait<T>>,
               user_repository: Box<dyn UserRepositoryTrait<T>>,
               profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
               secure_hasher: Box<dyn SecureHasher>,
               auth_connector: Box<dyn AuthConnector>) -> Self {
        UserProfileService {
            user_repository,
            profile_repository,
            transaction_creator,
            secure_hasher,
            auth_connector,
            marker: PhantomData::default(),
        }
    }

    pub async fn sign_up(&self, email: &str, password: &str, username: &str) -> Result<(String, String), UserProfileServiceError> {
        User::validate_email(email)?;
        User::validate_password(password)?;


        if self.user_repository.find_one_by_email(None, email).await.is_ok() {
            return Err(UserProfileServiceError::EmailAlreadyInUse);
        }

        let password_hash = self.secure_hasher.hash_password(password)?;

        let mut transaction = self.transaction_creator.create().await?;

        let user = User::register(Uuid::new_v4().to_string(), email.to_string(), password_hash, "user".to_string())?;
        self.user_repository.insert(Some(&mut transaction), &user).await?;

        let profile = Profile::new(Uuid::new_v4().to_string(), username.to_string(), None, None, None, None, user.get_id())?;
        self.profile_repository.create(Some(&mut transaction), &profile).await?;

        transaction.commit().await?;

        let session_id = self.auth_connector
            .create_session(user.get_id(), profile.get_id())
            .await?;

        Ok((profile.get_id(), session_id))
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<(String, String), UserProfileServiceError> {
        User::validate_email(email)?;
        User::validate_password(password)?;

        let user = self.user_repository.find_one_by_email(None, email).await?;

        self.secure_hasher.verify_password(password, user.get_password())?;

        let profile = self.profile_repository.find_by_user_id(None, user.get_id()).await?;

        let session_id = self.auth_connector
            .create_session(user.get_id(), profile.get_id())
            .await?;

        Ok((profile.get_id(), session_id))
    }
}