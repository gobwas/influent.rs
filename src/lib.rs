use std::collections::HashMap;

pub enum InfluxError {
    Unknown
}

pub enum Value<'a> {
    String(&'a str),
    Float64(f64),
    Integer64(i64),
    Boolean(bool)
}

pub struct Measurement<'a> {
    key: &'a str,
    timestamp: i8,
    fields: HashMap<&'a str, Value<'a>>
}

pub trait Serializer {
    fn serialize(&self, measurement: Measurement) -> String;
}

pub trait Client {
    fn write_many(self, Vec<Measurement>);
    fn write_one(self, Measurement);
}

pub struct HttpClient {
    serializer: Box<Serializer>
}

impl HttpClient {
    fn new(serializer: Box<Serializer>) -> HttpClient {
        HttpClient {
            serializer: serializer
        }
    }
}

impl Client for HttpClient {
    fn write_one(self, measurement: Measurement) {
        self.write_many(vec![measurement])
    }

    fn write_many(self, measurements: Vec<Measurement>) {
        let mut lines = Vec::new();

        for measurement in measurements {
            lines.push(self.serializer.serialize(measurement));
        }
    }
}
