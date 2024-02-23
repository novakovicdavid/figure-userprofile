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
    pub user_id: String,
    pub profile_id: String
}

impl Session {
    pub fn new(user_id: String, profile_id: String) -> Self {
        Self {
            user_id,
            profile_id,
        }
    }
}