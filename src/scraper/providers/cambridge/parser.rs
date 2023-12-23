use scraper::Html;

use super::statics::DEF_SELECTOR;

pub fn parse(doc: &str) -> Vec<String> {
    let document = Html::parse_document(doc);
    let defs = document.select(&DEF_SELECTOR);

    defs.into_iter()
        .map(|def| {
            let def = def.text().collect::<String>();
            def.trim()
                .strip_suffix(':')
                .map(String::from)
                .unwrap_or(def)
        })
        .collect()
}
