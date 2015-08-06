use ::measurement::Measurement;

pub mod http;

pub trait Client {
    fn write_many(&self, Vec<Measurement>, Option<Options>);
    fn write_one(&self, Measurement, Option<Options>);
}

pub struct Credentials<'a> {
    username: &'a str,
    password: &'a str,
    database: &'a str
}

pub enum Precision {
	Nanoseconds,
	Microseconds,
	Milliseconds,
	Seconds,
	Munutes,
	Hours
}

pub struct Options {
	precision: Precision,
	epoch:     Precision
}

pub enum ClientError {
	Syntax,
	Unknown
}