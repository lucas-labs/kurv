use anyhow::{Result, anyhow};
use log::{Record, Level, Metadata};

use crate::printth;

pub struct Logger;

impl Logger {
    pub fn init(max_level: Level) -> Result<()> {
        log::set_logger(&Logger)
            .map(|()| log::set_max_level(max_level.to_level_filter()))
            .map_err(|err| anyhow!("failed to set logger: {}", err))
    }

    fn level_theme(&self, level: Level) -> &str {
        match level {
            Level::Error => "error",
            Level::Warn =>  "warn",
            Level::Info =>  "info",  
            Level::Debug => "debug", 
            Level::Trace => "trace",
        }
    }
}

/// simple implementation of a themed logger
impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        let thm = self.level_theme(record.level());
        let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

        match record.level() {
            Level::Info => {
                printth!("<dim>{}</dim> <{thm}>⇝</{thm}>  {}", date, record.args());
            },
            _ => {
                printth!("<dim>{}</dim> <{thm}>⇝  {}</{thm}>", date, record.args());
            },
        }
    }

    fn flush(&self) {}   
}