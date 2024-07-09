pub use profile_entity::ProfileEntity;

mod profile_entity {
    use tokio_postgres::Row;

    use crate::application::errors::RepositoryError;
    use crate::domain::Profile;

    pub struct ProfileEntity {
        id: String,
        username: String,
        display_name: Option<String>,
        bio: Option<String>,
        banner: Option<String>,
        profile_picture: Option<String>,
        user_id: String,
    }

    impl TryFrom<Row> for ProfileEntity {
        type Error = RepositoryError;

        fn try_from(value: Row) -> Result<Self, Self::Error> {
            let id = value
                .try_get("id")
                .or_else(|_| value.try_get("profile_id"))?;

            let username: String = value.try_get("username")?;
            let display_name: Option<String> = value.try_get("display_name").ok();
            let bio: Option<String> = value.try_get("bio").ok();
            let banner: Option<String> = value.try_get("banner").ok();
            let profile_picture: Option<String> = value.try_get("profile_picture").ok();
            let user_id = value.try_get("user_id")?;

            Ok(Self {
                id,
                username,
                display_name,
                bio,
                banner,
                profile_picture,
                user_id,
            })
        }
    }

    impl From<ProfileEntity> for Profile {
        fn from(entity: ProfileEntity) -> Self {
            Profile {
                id: entity.id,
                username: entity.username,
                display_name: entity.display_name,
                bio: entity.bio,
                banner: entity.banner,
                profile_picture: entity.profile_picture,
                user_id: entity.user_id,
            }
        }
    }
}
