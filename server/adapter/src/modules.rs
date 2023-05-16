use kernel::{model::url::Url, repository::url::UrlRepository};

use crate::{persistence::mysql::Db, repository::DatabaseRepositoryImpl};

pub struct RepositoriesModule {
    url_repository: DatabaseRepositoryImpl<Url>,
}

pub trait RepositoriesModuleExt {
    type UrlRepo: UrlRepository;
    fn url_repository(&self) -> &Self::UrlRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UrlRepo = DatabaseRepositoryImpl<Url>;
    fn url_repository(&self) -> &Self::UrlRepo {
        &self.url_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let url_repository = DatabaseRepositoryImpl::<Url>::new(db);
        Self { url_repository }
    }
}
