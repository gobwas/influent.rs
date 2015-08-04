extern crate hyper;

use std::collections::BTreeMap;
use std::io::Read;

use hyper::Client as HyperClient;
use hyper::header::Connection;

pub enum InfluentError {
    Unknown
}

pub enum Value<'a> {
    String(&'a str),
    Float(f64),
    Integer(i64),
    Boolean(bool)
}

pub struct Measurement<'a> {
    key: &'a str,
    timestamp: Option<i32>,
    fields: BTreeMap<&'a str, Value<'a>>,
    tags: BTreeMap<&'a str, &'a str>
}

impl<'a> Measurement<'a> {
    pub fn new(key: &str) -> Measurement {
        Measurement {
            key: key,
            timestamp: None,
            fields: BTreeMap::new(),
            tags: BTreeMap::new()
        }
    }

    pub fn add_field(&mut self, field: &'a str, value: Value<'a>) {
        self.fields.insert(field, value);
    }

    pub fn add_tag(&mut self, tag: &'a str, value: &'a str) {
        self.tags.insert(tag, value);
    }

    pub fn set_timestamp(&mut self, timestamp: i32) {
        self.timestamp = Some(timestamp);
    }
}

pub struct Credentials<'a> {
    username: &'a str,
    password: &'a str,
    database: &'a str
}

pub trait Serializer {
    fn serialize(&self, measurement: &Measurement) -> String;
}

pub struct LineSerializer;

impl LineSerializer {
    pub fn new() -> LineSerializer {
        LineSerializer
    }
}

fn escape(s: &str) -> String {
    s
        .replace(" ", "\\ ")
        .replace(",", "\\,")
}

fn as_string(s: &str) -> String {
    vec![
        "\"".to_string(),
        s.replace("\"", "\\\""),
        "\"".to_string()
    ].connect("")
}

fn as_integer(i: &i64) -> String {
    i.to_string()
}

fn as_float(f: &f64) -> String {
    let s = f.to_string();

    match s.find(".") {
        Some(_) => s,
        None => vec![s, ".0".to_string()].connect("")
    }
}

fn as_boolean(b: &bool) -> String {
    if *b { "t".to_string() } else { "f".to_string() }
}

impl Serializer for LineSerializer {
    fn serialize(&self, measurement: &Measurement) -> String {
        let mut line = vec![escape(measurement.key)];

        for (tag, value) in measurement.tags.iter() {
            line.push(",".to_string());
            line.push(escape(tag));
            line.push("=".to_string());
            line.push(escape(value));
        }

        let mut was_spaced = false;

        for (field, value) in measurement.fields.iter() {
            line.push({if was_spaced { was_spaced = true; " " } else { "," }}.to_string());
            line.push(escape(field));
            line.push("=".to_string());

            match value {
                &Value::String(ref s)  => line.push(as_string(s)),
                &Value::Integer(ref i) => line.push(as_integer(i)),
                &Value::Float(ref f)   => line.push(as_float(f)),
                &Value::Boolean(ref b) => line.push(as_boolean(b))
            };
        }

        match measurement.timestamp {
            Some(t) => {
                line.push(" ".to_string());
                line.push(t.to_string());
            }
            _ => {}
        }

        line.connect("")
    }
}

pub trait Client {
    fn write_many(self, Vec<Measurement>);
    fn write_one(self, Measurement);
}

pub struct HttpClient<'a> {
    credentials: Credentials<'a>,
    serializer: Box<Serializer>,
    hosts: Vec<&'a str>
}

impl<'a> HttpClient<'a> {
    pub fn new(credentials: Credentials, serializer: Box<Serializer>) -> HttpClient {
        HttpClient {
            credentials: credentials,
            serializer: serializer,
            hosts: vec![]
        }
    }

    pub fn add_host(&mut self, host: &'a str) {
        self.hosts.push(host);
    }

    fn get_host(&self) -> &'a str {
        match self.hosts.first() {
            Some(host) => host,
            None => panic!("Could not get host")
        }
    }
}

impl<'a> Client for HttpClient<'a> {
    fn write_one(self, measurement: Measurement) {
        self.write_many(vec![measurement])
    }

    fn write_many(self, measurements: Vec<Measurement>) {
        let host = self.get_host();
        let client = HyperClient::new();

        for chunk in measurements.chunks(5000) {
            let mut lines = vec![];

            for measurement in chunk {
                lines.push(self.serializer.serialize(measurement));
            }

            lines.connect("\n");

            // todo 
            // add here credentials
            let mut res = client.post(host)
                .header(Connection::close())
                .send()
                .unwrap();

                // Read the Response.
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
        }
    }
}
