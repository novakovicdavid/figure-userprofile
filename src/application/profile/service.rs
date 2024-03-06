use std::marker::PhantomData;

use error_conversion_macro::ErrorEnum;
use thiserror::Error;

use crate::application::errors::RepositoryError;
use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::application::transaction::TransactionTrait;
use crate::domain::Profile;

pub struct ProfileService<T> {
    profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
    marker: PhantomData<T>,
}

#[derive(Debug, ErrorEnum, Error)]
pub enum ProfileServiceError {
    #[error(transparent)]
    UnexpectedError(anyhow::Error),

    #[error(transparent)]
    RepositoryError(RepositoryError),
}

impl<T> ProfileService<T> {
    pub fn new(profile_repository: Box<dyn ProfileRepositoryTrait<T>>) -> Self {
        Self {
            profile_repository,
            marker: PhantomData::default(),
        }
    }
}

impl<T> ProfileService<T> where T: TransactionTrait {
    pub async fn find_profile_by_id(&self, profile_id: String) -> Result<Profile, ProfileServiceError> {
        self
            .profile_repository.find_by_id(None, profile_id)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update_profile_by_id(&self, profile_id: String, display_name: Option<String>, bio: Option<String>) -> Result<(), ProfileServiceError> {
        self.profile_repository.update_profile_by_id(None, profile_id, display_name, bio)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_total_profiles_count(&self) -> Result<i64, ProfileServiceError> {
        self.profile_repository.get_total_profiles_count(None)
            .await
            .map_err(|e| e.into())
    }
}