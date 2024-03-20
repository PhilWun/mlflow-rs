use std::io::Write;
use std::sync::RwLock;

use chrono::Local;
use log::Log;
use log::{self, SetLoggerError};

pub struct ExperimentLogger<L: Log + 'static> {
    wrapped_logger: L,
    log: RwLock<Vec<u8>>,
}

impl<L: Log + 'static> Log for ExperimentLogger<L> {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.wrapped_logger.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let time = Local::now();

            writeln!(
                self.log.write().expect("could not get write lock for log"),
                "[{}][{:<5}]: {}",
                time.to_rfc3339_opts(chrono::SecondsFormat::Millis, false),
                record.level(),
                record.args()
            )
            .unwrap();
        }

        self.wrapped_logger.log(record);
    }

    fn flush(&self) {
        self.wrapped_logger.flush();
    }
}

impl<L: Log + 'static> ExperimentLogger<L> {
    pub fn init(wrapped_logger: L) -> Result<&'static Self, SetLoggerError> {
        let logger = Box::leak(Box::new(Self::build(wrapped_logger))); // create static reference of the new logger

        log::set_logger(logger)?;
        log::set_max_level(log::LevelFilter::Trace);

        Ok(logger)
    }

    pub fn build(wrapped_logger: L) -> Self {
        Self {
            wrapped_logger,
            log: RwLock::new(Vec::new()),
        }
    }

    pub fn build_static_reference(wrapped_logger: L) -> &'static Self {
        Box::leak(Box::new(Self::build(wrapped_logger)))
    }
}

impl<L: Log + 'static> ToString for ExperimentLogger<L> {
    fn to_string(&self) -> String {
        String::from_utf8(self.log.read().unwrap().clone())
            .unwrap_or("Error: could not convert Vec<u8> to String".to_owned())
    }
}
