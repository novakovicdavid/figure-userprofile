use std::marker::PhantomData;

use crate::application::error_handling::RepositoryError;
use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::domain::Profile;
use crate::application::transaction::TransactionTrait;

pub struct ProfileService<T> {
    profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
    marker: PhantomData<T>,
}

#[derive(Debug)]
pub enum ProfileServiceError {
    UnexpectedError(anyhow::Error),
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
    pub async fn find_profile_by_id(&self, profile_id: i64) -> Result<Profile, ProfileServiceError> {
        self
            .profile_repository.find_by_id(None, profile_id)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update_profile_by_id(&self, profile_id: i64, display_name: Option<String>, bio: Option<String>) -> Result<(), ProfileServiceError> {
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

impl From<RepositoryError> for ProfileServiceError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}