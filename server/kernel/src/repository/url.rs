use async_trait::async_trait;

use crate::model::{
    url::{NewUrl, ShortUrl, Url},
    Id,
};

#[async_trait]
pub trait UrlRepository {
    async fn retrieve(&self, id: &Id<Url>) -> anyhow::Result<Option<Url>>;
    async fn find_by_short(&self, short: &ShortUrl) -> anyhow::Result<Option<Url>>;
    async fn insert(&self, source: NewUrl) -> anyhow::Result<()>;
}
