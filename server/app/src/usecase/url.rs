use std::{format, sync::Arc};

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::{
    model::{
        url::{LongUrl, NewUrl, ShortUrl, Url},
        Id,
    },
    repository::url::UrlRepository,
};

#[derive(new)]
pub struct UrlUsecase<R: RepositoriesModuleExt> {
    repository: Arc<R>,
}

impl<R: RepositoriesModuleExt> UrlUsecase<R> {
    pub async fn get_short_url(&self, long_url: &str) -> anyhow::Result<Url> {
        let long: LongUrl = long_url.to_owned().try_into()?;
        let repo = self.repository.url_repository();
        if let Ok(Some(u)) = repo.find_by_long(&long).await {
            // すでに対応するshort urlがあればそれを返す
            Ok(u)
        } else {
            // 新規にshort urlを生成
            let mut short = ShortUrl::new(long.gen_hash());
            // 生成したshort urlが重複していないか確認
            let mut loop_cnt = 0;
            while let Ok(res) = repo.find_by_short(&short).await {
                if res.is_none() {
                    return repo
                        .insert(NewUrl::new(Id::<NewUrl>::gen(), long, short.clone()))
                        .await;
                }
                // 衝突したときの処理
                // まず, 10回以上衝突していたらエラーにする
                if loop_cnt > 10 {
                    anyhow::bail!(
                        "stop generating short url from {} because of too many tryings",
                        long.0
                    )
                }
                loop_cnt += 1;
                // URLにloop_cntを付け足してハッシュを再生成
                short = ShortUrl::new(LongUrl::new(format!("{}{}", long.0, loop_cnt)).gen_hash());
            }
            anyhow::bail!("failed to find url by short")
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use adapter::{modules::RepositoriesModule, persistence::mysql::Db};

    use super::*;

    #[tokio::test]
    async fn test_get_short_url() {
        dotenv::dotenv().ok();

        let db = Db::new().await;
        let repo = Arc::new(RepositoriesModule::new(db));
        let usecase = UrlUsecase::new(repo.clone());
        let long_url = "https://example.com";
        let shortend = usecase.get_short_url(long_url).await.unwrap();
        println!("shortened: {:?}", shortend);
        let url = repo
            .url_repository()
            .find_by_short(&ShortUrl::new(shortend.short.0))
            .await
            .unwrap();
        assert_eq!(url.unwrap().long.0, long_url);
    }
}
