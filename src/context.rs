use std::marker::PhantomData;
use crate::application::profile::repository::ProfileRepositoryTrait;
use crate::application::profile::service::ProfileServiceTrait;
use crate::application::user::repository::UserRepositoryTrait;
use crate::application::user::service::UserServiceTrait;
use crate::application::transaction::TransactionTrait;

pub trait ContextTrait: Send + Sync {
    type ServiceContext: ServiceContextTrait;
    type RepositoryContext: RepositoryContextTrait;
    fn new(service_context: Self::ServiceContext, repository_context: Self::RepositoryContext) -> Self;
    fn service_context(&self) -> &Self::ServiceContext;
    fn repository_context(&self) -> &Self::RepositoryContext;
}

pub struct Context<SC, RC> {
    service_context: SC,
    repository_context: RC,
}

impl<SC: ServiceContextTrait, RC: RepositoryContextTrait> ContextTrait for Context<SC, RC> {
    type ServiceContext = SC;
    type RepositoryContext = RC;

    fn new(service_context: Self::ServiceContext, repository_context: Self::RepositoryContext) -> Self {
        Context {
            service_context,
            repository_context,
        }
    }

    fn service_context(&self) -> &Self::ServiceContext {
        &self.service_context
    }
    fn repository_context(&self) -> &Self::RepositoryContext {
        &self.repository_context
    }
}

pub trait ServiceContextTrait: Send + Sync {
    type UserService: UserServiceTrait;
    type ProfileService: ProfileServiceTrait;
    fn user_service(&self) -> &Self::UserService;
    fn profile_service(&self) -> &Self::ProfileService;
}

pub struct ServiceContext<US, PS> {
    user_service: US,
    profile_service: PS,
}

impl<US, PS> ServiceContext<US, PS> {
    pub fn new(user_service: US, profile_service: PS) -> ServiceContext<US, PS> {
        ServiceContext {
            user_service,
            profile_service,
        }
    }
}

impl<US, PS> ServiceContextTrait for ServiceContext<US, PS>
    where US: UserServiceTrait, PS: ProfileServiceTrait {
    type UserService = US;
    type ProfileService = PS;

    fn user_service(&self) -> &Self::UserService {
        &self.user_service
    }

    fn profile_service(&self) -> &Self::ProfileService {
        &self.profile_service
    }
}

pub trait RepositoryContextTrait: Send + Sync {
    type Transaction: TransactionTrait;
    type UserRepository: UserRepositoryTrait<Self::Transaction>;
    type ProfileRepository: ProfileRepositoryTrait<Self::Transaction>;

    fn user_repository(&self) -> &Self::UserRepository;
    fn profile_repository(&self) -> &Self::ProfileRepository;
}

pub struct RepositoryContext<T, UR, PR> {
    marker: PhantomData<T>,
    user_repository: UR,
    profile_repository: PR,
}

impl<T, UR, PR> RepositoryContext<T, UR, PR> {
    pub fn new(
        user_repository: UR,
        profile_repository: PR,) -> RepositoryContext<T, UR, PR> {
        RepositoryContext {
            marker: PhantomData::default(),
            user_repository,
            profile_repository,
        }
    }
}

impl<T, UR, PR> RepositoryContextTrait for RepositoryContext<T, UR, PR>
    where T: TransactionTrait, UR: UserRepositoryTrait<T>, PR: ProfileRepositoryTrait<T> {
    type Transaction = T;
    type UserRepository = UR;
    type ProfileRepository = PR;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }

    fn profile_repository(&self) -> &Self::ProfileRepository {
        &self.profile_repository
    }
}