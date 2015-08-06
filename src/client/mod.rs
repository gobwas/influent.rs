use ::measurement::Measurement;

pub mod http;

pub trait Client {
    fn write_many(self, Vec<Measurement>);
    fn write_one(self, Measurement);
}

pub struct Credentials<'a> {
    username: &'a str,
    password: &'a str,
    database: &'a str
}