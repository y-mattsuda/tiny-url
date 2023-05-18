use async_trait::async_trait;
use kernel::{
    model::{
        url::{LongUrl, NewUrl, ShortUrl, Url},
        Id,
    },
    repository::url::UrlRepository,
};
use sqlx::{
    query_as,
    types::chrono::{DateTime, Local},
    FromRow,
};

use super::DatabaseRepositoryImpl;

#[derive(FromRow, Clone)]
pub struct UrlTabel {
    pub id: String,
    pub long: String,
    pub short: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl TryFrom<UrlTabel> for Url {
    type Error = anyhow::Error;
    fn try_from(ut: UrlTabel) -> Result<Self, Self::Error> {
        Ok(Url::new(
            ut.id.try_into()?,
            ut.long.try_into()?,
            ut.short.try_into()?,
            ut.created_at,
            ut.updated_at,
        ))
    }
}

impl TryFrom<NewUrl> for UrlTabel {
    type Error = anyhow::Error;
    fn try_from(u: NewUrl) -> Result<Self, Self::Error> {
        Ok(Self {
            id: u.id.value.to_string(),
            long: u.long.0,
            short: u.short.0,
            created_at: Local::now(),
            updated_at: Local::now(),
        })
    }
}

#[async_trait]
impl UrlRepository for DatabaseRepositoryImpl<Url> {
    async fn retrieve(&self, id: &Id<Url>) -> anyhow::Result<Option<Url>> {
        let pool = self.pool.0.clone();
        let url_table = query_as::<_, UrlTabel>("SELECT * FROM url WHERE id = ?")
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();
        match url_table {
            Some(ut) => Ok(Some(ut.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find_by_short(&self, short: &ShortUrl) -> anyhow::Result<Option<Url>> {
        let pool = self.pool.0.clone();
        let url_table = query_as::<_, UrlTabel>("SELECT * FROM url WHERE short = ?")
            .bind(short.0.clone())
            .fetch_one(&*pool)
            .await
            .ok();
        match url_table {
            Some(ut) => Ok(Some(ut.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find_by_long(&self, long: &LongUrl) -> anyhow::Result<Option<Url>> {
        let pool = self.pool.0.clone();
        let url_table = query_as::<_, UrlTabel>("SELECT * FROM url WHERE `long` = ?")
            .bind(long.0.clone())
            .fetch_one(&*pool)
            .await
            .ok();
        match url_table {
            Some(ut) => Ok(Some(ut.try_into()?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, source: NewUrl) -> anyhow::Result<Url> {
        let pool = self.pool.0.clone();
        let url_table: UrlTabel = source.try_into()?;
        let url: Url = url_table.clone().try_into()?;
        sqlx::query(
            "INSERT INTO url (id, `long`, short, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(url_table.id)
        .bind(url_table.long)
        .bind(url_table.short)
        .bind(url_table.created_at)
        .bind(url_table.updated_at)
        .execute(&*pool)
        .await?;
        Ok(url)
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use kernel::model::url::LongUrl;

    use crate::persistence::mysql::Db;

    use super::*;
    #[tokio::test]
    async fn test_url_repository() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let db = Db::new().await;
        let repo = DatabaseRepositoryImpl::<Url>::new(db);

        let long = LongUrl::new("https://www.google.com".to_string());
        let short = ShortUrl::new(long.gen_hash());
        let new_url = NewUrl::new(
            Id::<NewUrl>::gen(),
            LongUrl::new("https://www.google.com".to_string()),
            short.clone(),
        );

        repo.insert(new_url).await?;

        let found = repo.find_by_short(&short).await?.unwrap();
        assert_eq!(found.long, long);
        assert_eq!(found.short, short);

        Ok(())
    }
}
