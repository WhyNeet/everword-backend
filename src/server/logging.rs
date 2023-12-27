use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
    Config, Handle,
};

pub fn init() -> anyhow::Result<Handle> {
    let file = FileAppender::builder()
        .encoder(Box::<JsonEncoder>::default())
        .build("logs/server.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(file)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))?;

    Ok(log4rs::init_config(config)?)
}
