use ::measurement::Measurement;
use ::serializer::Serializer;
use ::client::{Client, Credentials, Options};
use std::collections::HashMap;
use self::hurl::{Hurl, Request, Response, Method, Auth};

pub mod hurl;

pub struct HttpClient<'a> {
    credentials: Credentials<'a>,
    serializer: Box<Serializer>,
    hurl: Box<Hurl>,
    hosts: Vec<&'a str>
}

impl<'a> HttpClient<'a> {
    pub fn new(credentials: Credentials, serializer: Box<Serializer>, hurl: Box<Hurl>) -> HttpClient {
        HttpClient {
            credentials: credentials,
            serializer: serializer,
            hurl: hurl,
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
    fn write_one(&self, measurement: Measurement, options: Option<Options>) {
        self.write_many(vec![measurement], options)
    }

    fn write_many(&self, measurements: Vec<Measurement>, options: Option<Options>) {
        let host = self.get_host();

        for chunk in measurements.chunks(5000) {
            let mut lines = Vec::new();

            for measurement in chunk {
                lines.push(self.serializer.serialize(measurement));
            }

            let mut query = HashMap::new();
            query.insert("db", self.credentials.database);

            let request = Request {
                url: &*{host.to_string() + "/write"},
                method: Method::POST,
                auth: Some(Auth {
                    username: self.credentials.username,
                    password: self.credentials.password
                }),
                query: Some(query),
                body: Some(lines.connect("\n"))
            };

            self.hurl.request(request);
        }
    }
}