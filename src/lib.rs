#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate failure;
extern crate fruently;
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serializable_enum;

use fruently::fluent::Fluent;
use fruently::forwardable::JsonForwardable;
use std::borrow::Cow;
use std::net::ToSocketAddrs;

use log::{Log, Metadata};

pub struct Logger<'a, A>
where
    A: ToSocketAddrs + Clone + Send + Sync,
{
    fluent: Fluent<'a, A>,
    level: log::Level,
}

#[derive(Serialize, Builder, Default, Clone, Debug)]
#[builder(setter(into))]
struct Record {
    level: Level,
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Parse(String),
}

serializable_enum! {
    #[derive(PartialEq, Clone, Debug)]
    enum Level {
        /// Error
        Error,
        /// Warn
        Warn,
        /// Info
        Info,
        /// Debug
        Debug,
        /// Trace
        Trace,
    }
    LevelVisitor
}

impl_as_ref_from_str! {
    Level {
        Error => "Error",
        Warn => "Warn",
        Info => "Info",
        Debug => "Debug",
        Trace => "Trace",
    }
    Error::Parse
}

impl Default for Level {
    fn default() -> Level {
        Level::Info
    }
}

impl From<log::Level> for Level {
    fn from(v: log::Level) -> Level {
        match v {
            log::Level::Error => Level::Error,
            log::Level::Warn => Level::Warn,
            log::Level::Info => Level::Info,
            log::Level::Debug => Level::Debug,
            log::Level::Trace => Level::Trace,
        }
    }
}

impl<'a, A> Logger<'a, A>
where
    A: ToSocketAddrs + Clone + Send + Sync,
{
    pub fn new<T>(addr: A, tag: T) -> Logger<'a, A>
    where
        T: Into<Cow<'a, str>>,
    {
        Logger {
            fluent: Fluent::new(addr, tag),
            level: log::Level::Debug,
        }
    }
}

impl<'a, A> Log for Logger<'a, A>
where
    A: ToSocketAddrs + Clone + Send + Sync,
{
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let record = RecordBuilder::default().level(record.level()).build();
            self.fluent.clone().post(record).unwrap();
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = Logger::new("127.0.0.1:24224", "test-index");
    }
}
