use async_trait::async_trait;

use crate::model::{
    url::{NewUrl, Url},
    Id,
};

#[async_trait]
pub trait UrlRepository {
    async fn retrive(&self, id: &Id<Url>) -> anyhow::Result<Option<Url>>;
    async fn find_by_short(&self, short: &str) -> anyhow::Result<Option<Url>>;
    async fn insert(&self, url: NewUrl) -> anyhow::Result<()>;
}
