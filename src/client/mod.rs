use ::measurement::Measurement;

pub mod http;

pub trait Client {
    fn write_many(&self, Vec<Measurement>, Option<Precision>) -> ClientWriteResult;
    fn write_one(&self, Measurement, Option<Precision>) -> ClientWriteResult;
    fn query(&self, String, Option<Precision>) -> ClientReadResult;
}

pub struct Credentials<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub database: &'a str
}

pub enum Precision {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Munutes,
    Hours
}

impl ToString for Precision {
    fn to_string(&self) -> String {
        let s = match (*self) {
            Precision::Nanoseconds  => "n",
            Precision::Microseconds => "u",
            Precision::Milliseconds => "ms",
            Precision::Seconds      => "s",
            Precision::Munutes      => "m",
            Precision::Hours        => "h"
        };

        s.to_string()
    }
}

pub type ClientWriteResult = Result<(), ClientError>;

// TODO: here parsing json?
pub type ClientReadResult = Result<String, ClientError>;

#[derive(Debug)]
pub enum ClientError {
    CouldNotComplete(String),
    Communication(String),
    Syntax(String),
    Unexpected(String),
    Unknown
}