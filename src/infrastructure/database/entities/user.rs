pub use user_entity::UserEntity;

mod user_entity {
    use tokio_postgres::Row;

    use crate::application::errors::RepositoryError;
    use crate::domain::User;

    pub struct UserEntity {
        id: String,
        email: String,
        password: String,
        role: String,
    }

    impl TryFrom<Row> for UserEntity {
        type Error = RepositoryError;

        fn try_from(value: Row) -> Result<Self, Self::Error> {
            let id = value
                .try_get("id")
                .or_else(|_| value.try_get("user_id"))?;

            let email = value.try_get("email")?;
            let password = value.try_get("password")?;
            let role = value.try_get("role")?;

            Ok(Self {
                id,
                email,
                password,
                role,
            })
        }
    }

    impl From<UserEntity> for User {
        fn from(entity: UserEntity) -> Self {
            Self {
                id: entity.id,
                email: entity.email,
                password: entity.password,
                role: entity.role,
                password_resets: Vec::new(),
            }
        }
    }
}