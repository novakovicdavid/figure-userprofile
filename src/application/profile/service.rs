use std::marker::PhantomData;
use async_trait::async_trait;

use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::domain::Profile;
use crate::application::transaction::TransactionTrait;
use crate::application::server_errors::ServerError;

pub struct ProfileService<T> {
    profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
    marker: PhantomData<T>,
}

impl<T: TransactionTrait> ProfileService<T> {
    pub fn new(profile_repository: Box<dyn ProfileRepositoryTrait<T>>) -> Self {
        Self {
            profile_repository,
            marker: PhantomData::default(),
        }
    }
}

#[async_trait]
pub trait ProfileServiceTrait: Send + Sync {
    async fn find_profile_by_id(&self, profile_id: i64) -> Result<Profile, ServerError>;
    async fn update_profile_by_id(&self, profile_id: i64, display_name: Option<String>, bio: Option<String>) -> Result<(), ServerError>;
    async fn get_total_profiles_count(&self) -> Result<i64, ServerError>;
}

#[async_trait]
impl<T> ProfileServiceTrait for ProfileService<T> where T: TransactionTrait {
    async fn find_profile_by_id(&self, profile_id: i64) -> Result<Profile, ServerError> {
        self.profile_repository.find_by_id(None, profile_id).await
    }

    async fn update_profile_by_id(&self, profile_id: i64, display_name: Option<String>, bio: Option<String>) -> Result<(), ServerError> {
        self.profile_repository.update_profile_by_id(None, profile_id, display_name, bio).await
    }

    async fn get_total_profiles_count(&self) -> Result<i64, ServerError> {
        self.profile_repository.get_total_profiles_count(None).await
    }
}