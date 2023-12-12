use sqlx::{Pool, Postgres};

struct EventRepository {
    db: Pool<Postgres>,
}

impl EventRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            db: pool
        }
    }
}