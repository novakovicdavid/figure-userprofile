#[derive(Clone)]
pub struct SessionOption {
    pub session: Option<Session>,
}

impl SessionOption {
    pub fn new() -> Self {
        Self {
            session: None
        }
    }

    pub fn from(session: Session) -> Self {
        SessionOption {
            session: Some(session)
        }
    }
}

#[derive(Clone)]
pub struct Session {
    pub user_id: i64,
    pub profile_id: i64
}

impl Session {
    pub fn new(user_id: i64, profile_id: i64) -> Self {
        Self {
            user_id,
            profile_id,
        }
    }
}