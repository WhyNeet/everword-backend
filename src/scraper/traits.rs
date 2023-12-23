use async_trait::async_trait;

#[async_trait]
pub trait Scrape {
    async fn scrape(word: &str) -> anyhow::Result<Vec<String>>;
}
