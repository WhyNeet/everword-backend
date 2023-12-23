use std::collections::HashMap;

use scraper::Html;

use crate::dict::model::Defenition;

use super::statics::{
    DEF_BLOCK_SELECTOR, DEF_ENTRY_SELECTOR, DEF_EXAMPLES_SELECTOR, DEF_PART_OF_SPEECH_SELECTOR,
};

pub fn parse(doc: &str) -> HashMap<String, Vec<Defenition>> {
    let document = Html::parse_document(doc);
    let defs = document.select(&DEF_ENTRY_SELECTOR);

    defs.into_iter()
        .fold(HashMap::<String, Vec<Defenition>>::new(), |mut acc, def| {
            let mut text = def.select(&DEF_BLOCK_SELECTOR);
            let text = text.next().unwrap().text().collect::<String>();
            let text = text
                .trim()
                .strip_suffix(':')
                .map(String::from)
                .unwrap_or(text);

            let mut part_of_speech = def.select(&DEF_PART_OF_SPEECH_SELECTOR);
            let part_of_speech = part_of_speech.next().unwrap().text().collect::<String>();

            let examples = def.select(&DEF_EXAMPLES_SELECTOR);
            let examples = examples
                .into_iter()
                .map(|example| example.text().collect::<String>().trim().to_string())
                .collect::<Vec<String>>();

            let defention = Defenition::new(text, examples);

            acc.entry(part_of_speech)
                .or_insert(Vec::new())
                .push(defention);

            acc
        })
}
