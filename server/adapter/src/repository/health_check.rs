use std::sync::Arc;

use anyhow::anyhow;

use crate::persistence::mysql::Db;

pub struct HealthCheckRepository {
    db: Arc<Db>,
}

impl HealthCheckRepository {
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }

    pub async fn check_db_conn(&self) -> anyhow::Result<()> {
        let pool = self.db.0.clone();
        pool.try_acquire()
            .map(|_| ())
            .ok_or(anyhow!("Failed to connect to database."))
    }
}
