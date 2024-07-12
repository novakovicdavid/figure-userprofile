use error_conversion_macro::ErrorEnum;
use thiserror::Error;

use crate::application::errors::RepositoryError;
use crate::application::repository_traits::read::profile_repository::ProfileRepository;
use crate::domain::Profile;

pub struct ProfileService {
    profile_repository: Box<dyn ProfileRepository>,
}

#[derive(Debug, ErrorEnum, Error)]
pub enum ProfileServiceError {
    #[error(transparent)]
    UnexpectedError(anyhow::Error),

    #[error(transparent)]
    RepositoryError(RepositoryError),
}

impl ProfileService {
    pub fn new(profile_repository: Box<dyn ProfileRepository>) -> Self {
        Self {
            profile_repository,
        }
    }
}

impl ProfileService {
    pub async fn find_profile_by_id(&self, profile_id: String) -> Result<Profile, ProfileServiceError> {
        self
            .profile_repository.find_by_id(profile_id)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update_profile_by_id(&self, profile_id: String, display_name: Option<String>, bio: Option<String>) -> Result<(), ProfileServiceError> {
        self.profile_repository.update_profile_by_id(profile_id, display_name, bio)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_total_profiles_count(&self) -> Result<i64, ProfileServiceError> {
        self.profile_repository.get_total_profiles_count()
            .await
            .map_err(|e| e.into())
    }
}