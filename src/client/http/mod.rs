use ::measurement::Measurement;
use ::serializer::Serializer;
use ::client::{Precision, Client, Credentials};
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
    fn write_one(&self, measurement: Measurement, precision: Option<Precision>) {
        self.write_many(vec![measurement], precision)
    }

    fn write_many(&self, measurements: Vec<Measurement>, precision: Option<Precision>) {
        let host = self.get_host();

        for chunk in measurements.chunks(5000) {
            let mut lines = Vec::new();

            for measurement in chunk {
                lines.push(self.serializer.serialize(measurement));
            }

            let mut query = HashMap::new();
            query.insert("db", self.credentials.database.to_string());

            match precision {
                Some(ref precision) => {
                    query.insert("precision", precision.to_string());
                }
                _ => {}
            };

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



#[cfg(test)]
mod tests {
    use ::serializer::Serializer;
    use ::client::{Client};
    use super::HttpClient;
    use ::client::{Credentials, Precision};
    use ::client::http::hurl::{Hurl, Request, Response, HurlResult};
    use ::measurement::Measurement;
    use std::cell::Cell;
    use std::clone::Clone;

    const serialized : &'static str = "serialized";

    struct MockSerializer {
        serialize_count: Cell<u16>
    }

    impl MockSerializer {
        fn new() -> MockSerializer {
            MockSerializer {
                serialize_count: Cell::new(0)
            }
        }
    }

    impl Serializer for MockSerializer {
        fn serialize(&self, measurement: &Measurement) -> String {
            println!("serializing: {:?}", measurement);
            self.serialize_count.set(self.serialize_count.get() + 1);
            serialized.to_string()
        }
    }

    struct MockHurl {
        request_count: Cell<u16>,
        result: Box<Fn() -> HurlResult>
    }

    impl MockHurl {
        fn new(result: Box<Fn() -> HurlResult>) -> MockHurl {
            MockHurl {
                request_count: Cell::new(0),
                result: result
            }
        }
    }

    impl Hurl for MockHurl {
        fn request(&self, req: Request) -> HurlResult {
            self.request_count.set(self.request_count.get() + 1);
            println!("sending: {:?}", req);
            let ref f = self.result;
            f()
        }
    }

    fn before<'a>(result: Box<Fn() -> HurlResult>) -> HttpClient<'a> {        
        let credentials = Credentials {
            username: "gobwas",
            password: "1234",
            database: "test"
        };

        let serializer = MockSerializer::new();
        let hurl = MockHurl::new(result);

        HttpClient::new(credentials, Box::new(serializer), Box::new(hurl))
    }

    #[test]
    fn test_write_one() {
        let mut client = before(Box::new(|| Ok(Response { status: 200, body: "Ok".to_string() })));
        client.add_host("http://localhost:8086");
        client.write_one(Measurement::new("key"), Some(Precision::Nanoseconds));
    }

    #[test]
    fn test_write_many() {
        let mut client = before(Box::new(|| Ok(Response { status: 200, body: "Ok".to_string() })));
        client.add_host("http://localhost:8086");
        client.write_many(vec!(Measurement::new("key")), Some(Precision::Nanoseconds));
    }
}



