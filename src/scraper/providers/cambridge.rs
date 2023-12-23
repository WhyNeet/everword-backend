use async_trait::async_trait;
use scraper::Html;

use crate::scraper::traits::Scrape;

use super::statics::DEF_SELECTOR;

pub struct CambridgeScraper;

#[async_trait]
impl Scrape for CambridgeScraper {
    async fn scrape(word: &str) -> anyhow::Result<Vec<String>> {
        let result = reqwest::get(format!(
            "https://dictionary.cambridge.org/us/dictionary/english/{word}"
        ))
        .await?
        .text()
        .await?;

        let document = Html::parse_document(&result);
        let defs = document.select(&DEF_SELECTOR);

        Ok(defs
            .into_iter()
            .map(|def| {
                let def = def.text().collect::<String>();
                def.trim()
                    .strip_suffix(':')
                    .map(String::from)
                    .unwrap_or(def)
            })
            .collect())
    }
}
