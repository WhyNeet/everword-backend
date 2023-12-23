use backend::scraper::cambridge::{self, statics::DEF_SELECTOR};
use scraper::Html;

#[tokio::test]
async fn defs_selector_works() {
    let doc = reqwest::get("https://dictionary.cambridge.org/us/dictionary/english/word")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(&doc);
    let mut defs = document.select(&DEF_SELECTOR);

    assert!(defs.next().is_some());
}

#[test]
fn parser_works() {
    let document = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Document</title>
    </head>
    <body>
      <div class="def">first</div>
      <div class="def">second</div>
      <div class="def">third</div>
    </body>
    </html>
    "#;

    let defs = cambridge::parser::parse(document);

    assert_eq!(
        defs,
        vec![
            String::from("first"),
            String::from("second"),
            String::from("third")
        ]
    );
}
