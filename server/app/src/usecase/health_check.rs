use std::sync::Arc;

use adapter::repository::health_check::HealthCheckRepository;

pub struct HealthCheckUsecase {
    repository: Arc<HealthCheckRepository>,
}

impl HealthCheckUsecase {
    pub fn new(repository: HealthCheckRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }

    pub async fn diagnose_db_conn(&self) -> anyhow::Result<()> {
        self.repository.check_db_conn().await
    }
}
