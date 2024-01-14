use std::marker::PhantomData;
use async_trait::async_trait;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Algorithm::Argon2id;
use argon2::password_hash::{Error, SaltString};
use rand_core::OsRng;

use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::application::server_errors::ServerError;
use crate::application::user::repository::UserRepositoryTrait;
use crate::domain::User;
use crate::infrastructure::secure_rand_generator::RandomNumberGenerator;
use crate::application::transaction::{TransactionManagerTrait, TransactionTrait};


pub struct UserService<T> {
    transaction_creator: Box<dyn TransactionManagerTrait<T>>,
    marker: PhantomData<T>,
    user_repository: Box<dyn UserRepositoryTrait<T>>,
    profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
    secure_random_generator: Box<dyn RandomNumberGenerator>,
}

impl<T> UserService<T> where T: TransactionTrait
{
    pub fn new(transaction_creator: Box<dyn TransactionManagerTrait<T>>,
               user_repository: Box<dyn UserRepositoryTrait<T>>,
               profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
               secure_random_generator: Box<dyn RandomNumberGenerator>) -> Self {
        UserService {
            user_repository,
            profile_repository,
            transaction_creator,
            secure_random_generator,
            marker: PhantomData::default(),
        }
    }
}

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn sign_up(&self, email: &str, password: &str, username: &str) -> Result<(i64, String), ServerError>;
    async fn sign_in(&self, email: &str, password: &str) -> Result<(i64, String), ServerError>;
}

#[async_trait]
impl<T> UserServiceTrait for UserService<T> where T: TransactionTrait
{
    async fn sign_up(&self, email: &str, password: &str, username: &str) -> Result<(i64, String), ServerError> {
        User::validate_email(email)?;
        User::validate_password(password)?;


        if self.user_repository.find_one_by_email(None, email).await.is_ok() {
            return Err(ServerError::EmailAlreadyInUse);
        }

        let password_hash = hash_password(password)?;

        let mut transaction = self.transaction_creator.create().await?;

        let user = User::new(0, email.to_string(), password_hash, "user".to_string())?;
        let user = self.user_repository.create(Some(&mut transaction), user).await?;

        transaction.commit().await?;

        Ok((1, "d".to_string()))
    }

    async fn sign_in(&self, email: &str, password: &str) -> Result<(i64, String), ServerError> {
        User::validate_email(email)?;
        User::validate_password(password)?;

        let user = self.user_repository.find_one_by_email(None, email).await?;

        let parsed_hash = PasswordHash::new(user.get_password())
            .map_err(|e| ServerError::InternalError(e.into()))?;

        verify_password(password, parsed_hash)?;

        let profile = self.profile_repository.find_by_user_id(None, user.get_id()).await?;

        Ok((1, "d".to_string()))
    }
}

pub fn hash_password(password: &str) -> Result<String, ServerError> {
    let password_salt = SaltString::generate(&mut OsRng);
    let argon2_params = Params::new(8192, 5, 1, Some(32))
        .map_err(|e| ServerError::InternalError(e.into()))?;

    let password_hash = Argon2::new(Argon2id, argon2::Version::V0x13, argon2_params).hash_password(password.as_ref(), &password_salt)
        .map_err(|e| ServerError::InternalError(e.into()))?;
    Ok(password_hash.to_string())
}

fn verify_password(password: &str, saved_hash: PasswordHash) -> Result<(), ServerError> {
    Argon2::default().verify_password(password.as_bytes(), &saved_hash)
        .map_err(|e| match e {
            Error::Password => ServerError::WrongPassword,
            _ => ServerError::InternalError(e.into())
        })
}