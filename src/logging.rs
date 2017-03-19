use log::{Log, LogRecord, LogLevel, LogMetadata, LogLevelFilter};
use std::io::{self, Write};
use error::ResultExt;


pub struct Logger;

impl Logger {
    pub fn init() -> ::Result<()> {
        Ok(::log::set_logger(|max_log_filter| {
                                 max_log_filter.set(LogLevelFilter::Info);
                                 Box::new(Logger)
                             })?)
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if !self.enabled(record.metadata()) {
            return;
        }
        writeln!(io::stderr(), "[{}]: {}", record.level(), record.args())
            .chain_err(|| "failed writing to stderr")
            .unwrap();
    }
}
