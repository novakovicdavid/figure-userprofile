use std::marker::PhantomData;
use async_trait::async_trait;
use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::domain::profile::Profile;
use crate::infrastructure::traits::TransactionTrait;
use crate::server_errors::ServerError;

pub struct ProfileService<T, P> {
    profile_repository: P,
    marker: PhantomData<T>,
}

impl<T: TransactionTrait, P: ProfileRepositoryTrait<T>> ProfileService<T, P> {
    pub fn new(profile_repository: P) -> Self {
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
impl<T, P> ProfileServiceTrait for ProfileService<T, P>
    where T: TransactionTrait, P: ProfileRepositoryTrait<T> {
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