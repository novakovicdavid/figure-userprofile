use std::marker::PhantomData;

use crate::application::error_handling::RepositoryError;
use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::application::transaction::{TransactionError, TransactionManagerTrait, TransactionTrait};
use crate::application::user::repository::UserRepositoryTrait;
use crate::domain::User;
use crate::domain::user::UserDomainError;
use crate::infrastructure::secure_hasher::{SecureHasher, SecureHasherError};
use crate::infrastructure::secure_rand_generator::RandomNumberGenerator;

pub struct UserService<T> {
    transaction_creator: Box<dyn TransactionManagerTrait<T>>,
    marker: PhantomData<T>,
    user_repository: Box<dyn UserRepositoryTrait<T>>,
    profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
    secure_random_generator: Box<dyn RandomNumberGenerator>,
    secure_hasher: Box<dyn SecureHasher>,
}

#[derive(Debug)]
pub enum UserServiceError {
    EmailAlreadyInUse,
    WrongPassword,

    UserDomainError(UserDomainError),

    RepositoryError(RepositoryError),
    TransactionError(TransactionError),
    SecureHasherError(SecureHasherError),

    UnexpectedError(anyhow::Error),
}

impl<T> UserService<T> where T: TransactionTrait
{
    pub fn new(transaction_creator: Box<dyn TransactionManagerTrait<T>>,
               user_repository: Box<dyn UserRepositoryTrait<T>>,
               profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
               secure_random_generator: Box<dyn RandomNumberGenerator>,
               secure_hasher: Box<dyn SecureHasher>) -> Self {
        UserService {
            user_repository,
            profile_repository,
            transaction_creator,
            secure_random_generator,
            secure_hasher,
            marker: PhantomData::default(),
        }
    }

    pub async fn sign_up(&self, email: &str, password: &str, username: &str) -> Result<(i64, String), UserServiceError> {
        User::validate_email(email)?;
        User::validate_password(password)?;


        if self.user_repository.find_one_by_email(None, email).await.is_ok() {
            return Err(UserServiceError::EmailAlreadyInUse);
        }

        let password_hash = self.secure_hasher.hash_password(password)?;

        let mut transaction = self.transaction_creator.create().await?;

        let user = User::new(0, email.to_string(), password_hash, "user".to_string())?;
        let user = self.user_repository.create(Some(&mut transaction), user).await?;

        transaction.commit().await?;

        Ok((1, "d".to_string()))
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<(i64, String), UserServiceError> {
        User::validate_email(email)?;
        User::validate_password(password)?;

        let user = self.user_repository.find_one_by_email(None, email).await?;

        self.secure_hasher.verify_password(password, user.get_password())?;

        let profile = self.profile_repository.find_by_user_id(None, user.get_id()).await?;

        Ok((1, "d".to_string()))
    }
}

impl From<UserDomainError> for UserServiceError {
    fn from(value: UserDomainError) -> Self {
        UserServiceError::UserDomainError(value)
    }
}

impl From<SecureHasherError> for UserServiceError {
    fn from(value: SecureHasherError) -> Self {
        UserServiceError::SecureHasherError(value)
    }
}

impl From<TransactionError> for UserServiceError {
    fn from(value: TransactionError) -> Self {
        UserServiceError::TransactionError(value)
    }
}

impl From<RepositoryError> for UserServiceError {
    fn from(value: RepositoryError) -> Self {
        UserServiceError::RepositoryError(value)
    }
}