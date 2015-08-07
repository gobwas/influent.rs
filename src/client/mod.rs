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

impl ToString for Precision {
	fn to_string(&self) -> String {
		let s = match (*self) {
			Precision::Nanoseconds  => "n",
			Precision::Microseconds => "u",
			Precision::Milliseconds => "ms",
			Precision::Seconds 	    => "s",
			Precision::Munutes 	    => "m",
			Precision::Hours 	    => "h"
		};

		s.to_string()
	}
}

pub struct Options {
	precision: Option<Precision>,
	epoch:     Option<Precision>
}

pub enum ClientError {
	Syntax,
	Unknown
}