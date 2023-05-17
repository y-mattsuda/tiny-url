use std::sync::Arc;

use adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::mysql::Db,
    repository::health_check::HealthCheckRepository,
};
use app::usecase::{health_check::HealthCheckUsecase, url::UrlUsecase};

pub struct Modules {
    health_check_use_case: HealthCheckUsecase,
    url_use_case: UrlUsecase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUsecase;
    fn url_use_case(&self) -> &UrlUsecase<Self::RepositoriesModule>;
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;
        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));
        let health_check_use_case = HealthCheckUsecase::new(HealthCheckRepository::new(db));
        let url_use_case = UrlUsecase::new(repositories_module);

        Self {
            health_check_use_case,
            url_use_case,
        }
    }
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUsecase {
        &self.health_check_use_case
    }

    fn url_use_case(&self) -> &UrlUsecase<Self::RepositoriesModule> {
        &self.url_use_case
    }
}
