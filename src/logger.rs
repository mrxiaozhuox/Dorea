use log::SetLoggerError;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config, Handle,
};

pub(crate) fn init_logger() -> Result<Handle, SetLoggerError> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{l}] {d} - {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(
            Root::builder()
                .appender("console")
                .build(log::LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config)
}
