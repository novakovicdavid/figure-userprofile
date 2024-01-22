use std::marker::PhantomData;

use crate::application::error_handling::RepositoryError;
use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::application::transaction::{TransactionError, TransactionManagerTrait, TransactionTrait};
use crate::application::user_profile::repository::UserRepositoryTrait;
use crate::domain::{Profile, User};
use crate::domain::profile::ProfileDomainError;
use crate::domain::user::UserDomainError;
use crate::infrastructure::secure_hasher::{SecureHasher, SecureHasherError};
use crate::infrastructure::secure_rand_generator::RandomNumberGenerator;

pub struct UserProfileService<T> {
    transaction_creator: Box<dyn TransactionManagerTrait<T>>,
    marker: PhantomData<T>,
    user_repository: Box<dyn UserRepositoryTrait<T>>,
    profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
    secure_random_generator: Box<dyn RandomNumberGenerator>,
    secure_hasher: Box<dyn SecureHasher>,
}

#[derive(Debug)]
pub enum UserProfileServiceError {
    EmailAlreadyInUse,
    WrongPassword,

    UserDomainError(UserDomainError),
    ProfileDomainError(ProfileDomainError),

    RepositoryError(RepositoryError),
    TransactionError(TransactionError),
    SecureHasherError(SecureHasherError),

    UnexpectedError(anyhow::Error),
}

impl<T> UserProfileService<T> where T: TransactionTrait
{
    pub fn new(transaction_creator: Box<dyn TransactionManagerTrait<T>>,
               user_repository: Box<dyn UserRepositoryTrait<T>>,
               profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
               secure_random_generator: Box<dyn RandomNumberGenerator>,
               secure_hasher: Box<dyn SecureHasher>) -> Self {
        UserProfileService {
            user_repository,
            profile_repository,
            transaction_creator,
            secure_random_generator,
            secure_hasher,
            marker: PhantomData::default(),
        }
    }

    pub async fn sign_up(&self, email: &str, password: &str, username: &str) -> Result<(i64, String), UserProfileServiceError> {
        User::validate_email(email)?;
        User::validate_password(password)?;


        if self.user_repository.find_one_by_email(None, email).await.is_ok() {
            return Err(UserProfileServiceError::EmailAlreadyInUse);
        }

        let password_hash = self.secure_hasher.hash_password(password)?;

        let mut transaction = self.transaction_creator.create().await?;

        let user = User::new(0, email.to_string(), password_hash, "user".to_string())?;
        let user = self.user_repository.create(Some(&mut transaction), user).await?;

        let profile = Profile::new(0, username.to_string(), None, None, None, None, user.get_id())?;
        let profile = self.profile_repository.create(Some(&mut transaction), profile).await?;

        transaction.commit().await?;



        Ok((profile.get_id(), "".to_string()))
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<(i64, String), UserProfileServiceError> {
        User::validate_email(email)?;
        User::validate_password(password)?;

        let user = self.user_repository.find_one_by_email(None, email).await?;

        self.secure_hasher.verify_password(password, user.get_password())?;

        let profile = self.profile_repository.find_by_user_id(None, user.get_id()).await?;

        Ok((1, "d".to_string()))
    }
}

impl From<UserDomainError> for UserProfileServiceError {
    fn from(value: UserDomainError) -> Self {
        UserProfileServiceError::UserDomainError(value)
    }
}

impl From<SecureHasherError> for UserProfileServiceError {
    fn from(value: SecureHasherError) -> Self {
        UserProfileServiceError::SecureHasherError(value)
    }
}

impl From<TransactionError> for UserProfileServiceError {
    fn from(value: TransactionError) -> Self {
        UserProfileServiceError::TransactionError(value)
    }
}

impl From<RepositoryError> for UserProfileServiceError {
    fn from(value: RepositoryError) -> Self {
        UserProfileServiceError::RepositoryError(value)
    }
}

impl From<ProfileDomainError> for UserProfileServiceError {
    fn from(value: ProfileDomainError) -> Self {
        Self::ProfileDomainError(value)
    }
}