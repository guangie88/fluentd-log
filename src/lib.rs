extern crate fruently;
#[macro_use]
extern crate log;

use fruently::fluent::Fluent;
use std::borrow::Cow;
use std::net::ToSocketAddrs;

pub struct Logger<'a, A>
where
    A: ToSocketAddrs,
{
    fluent: Fluent<'a, A>,
}

impl<'a, A> Logger<'a, A>
where
    A: ToSocketAddrs,
{
    pub fn new<T>(addr: A, tag: T) -> Logger<'a, A>
    where
        T: Into<Cow<'a, str>>,
    {
        Logger {
            fluent: Fluent::new(addr, tag),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = Logger::new("127.0.0.1:24224", "test-index");
    }
}
