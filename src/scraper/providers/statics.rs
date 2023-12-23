use lazy_static::lazy_static;
use scraper::Selector;

lazy_static! {
    pub static ref DEF_SELECTOR: Selector = Selector::parse("div.def").unwrap();
}
