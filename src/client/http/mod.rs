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
            query.insert("db", self.credentials.database.to_string());

            match options {
                Some(Options { precision: Some(ref precision), .. }) => {
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
    use super::HttpClient;
    use ::client::{Credentials};
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
            let ref f = self.result;
            f()
        }
    }

    fn before<'a>(result: HurlResult) -> HttpClient<'a> {        
        let credentials = Credentials {
            username: "gobwas",
            password: "1234",
            database: "test"
        };

        let serializer = MockSerializer::new();
        let hurl = MockHurl::new(Box::new(|| Err("err".to_string())));

        HttpClient::new(credentials, Box::new(serializer), Box::new(hurl))
    }

    #[test]
    fn test_write_one() {

    }
}



