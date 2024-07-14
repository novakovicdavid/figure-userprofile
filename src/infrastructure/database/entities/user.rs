pub use user_entity::UserEntity;

mod user_entity {
    use tokio_postgres::Row;

    use crate::application::errors::RepositoryError;
    use crate::domain::User;
    use crate::domain::user::user::ResetPasswordRequest;

    pub struct UserEntity {
        pub id: String,
        pub email: String,
        pub password: String,
        pub role: String,
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

    impl UserEntity {
        pub fn into_user(self, reset_password_requests: Vec<ResetPasswordRequest>) -> User {
            User::new(
                self.id,
                self.email,
                self.password,
                self.role,
                reset_password_requests,
            )
        }
    }
}