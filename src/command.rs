use std::net::TcpStream;
use std::{thread, time};

#[derive(Default)]
pub struct Builder {
    target: String,
    interval: String,
    max_retry: String,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = target.into();
        self
    }

    pub fn interval(mut self, interval: impl Into<String>) -> Self {
        self.interval = interval.into();
        self
    }

    pub fn max_retry(mut self, max_retry: impl Into<String>) -> Self {
        self.max_retry = max_retry.into();
        self
    }

    pub fn build(self) -> Command {
        Command {
            target: self.target,
            interval: self.interval.parse().unwrap(),
            max_retry: self.max_retry.parse().expect("parse max_retry"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    RetryExceed,
}

#[derive(Debug)]
pub struct Command {
    target: String,
    interval: u64,
    max_retry: u64,
}

impl Command {
    pub fn run(self) -> Result<(), Error> {
        self.try_connect()
    }

    fn try_connect(&self) -> Result<(), Error> {
        use std::io::ErrorKind;

        let mut retry_count: u64 = 0;
        loop {
            retry_count = retry_count + 1;
            match TcpStream::connect(self.target.as_str()) {
                Ok(_) => return Ok(()),
                Err(ioerr) => match ioerr.kind() {
                    ErrorKind::ConnectionRefused => println!(
                        "connection refused... attempt:{}/{}",
                        retry_count, self.max_retry
                    ),
                    _ => return Err(Error::IOError(ioerr)),
                },
            }
            if retry_count >= self.max_retry {
                return Err(Error::RetryExceed);
            }
            thread::sleep(time::Duration::from_secs(self.interval))
        }
    }
}
