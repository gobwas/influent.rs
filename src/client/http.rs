use ::measurement::Measurement;
use ::serializer::Serializer;
use ::client::{Precision, Client, Credentials, ClientError, ClientReadResult, ClientWriteResult};
use ::hurl::{Hurl, Request, Method, Auth};
use std::collections::HashMap;
use futures::{Future, stream, Stream};

const MAX_BATCH: u16 = 5000;

pub enum WriteStatus {
    Success,
    CouldNotComplete,
}

// fixme
pub struct Options {
    pub max_batch: Option<u16>,
    pub precision: Option<Precision>,

    pub epoch: Option<Precision>,
    pub chunk_size: Option<u16>
}

pub struct HttpClient {
    credentials: Credentials,
    serializer: Box<Serializer + Send + Sync>,
    hurl: Box<Hurl + Send + Sync>,
    hosts: Vec<String>,
    pub max_batch: u16
}

impl HttpClient {
    pub fn new(credentials: Credentials, serializer: Box<Serializer + Send + Sync>, hurl: Box<Hurl + Send + Sync>) -> HttpClient {
        HttpClient {
            credentials: credentials,
            serializer: serializer,
            hurl: hurl,
            hosts: vec![],
            max_batch: MAX_BATCH
        }
    }

    pub fn add_host<S>(&mut self, host: S) where S: Into<String> {
        self.hosts.push(host.into());
    }

    fn get_host(&self) -> String {
        match self.hosts.first() {
            Some(host) => host.to_string(),
            None => panic!("Could not get host")
        }
    }
}

impl Client for HttpClient {
    fn query(&self, q: String, epoch: Option<Precision>) -> ClientReadResult {
        let host = self.get_host();

        let mut query = HashMap::new();
        query.insert(String::from("db"), self.credentials.database.to_string());
        query.insert(String::from("q"), q);

        if let Some(ref epoch) = epoch {
            query.insert(String::from("epoch"), epoch.to_string());
        }

        let request = Request {
            url: format!("{}/query", host.to_string()),
            method: Method::GET,
            auth: Some(Auth {
                username: self.credentials.username.clone(),
                password: self.credentials.password.clone()
            }),
            query: Some(query),
            body: None
        };

        Box::new(self.hurl.request(request).then(|res| {
            match res {
                Ok(ref resp) if resp.status == 200 => Ok(resp.to_string()),
                Ok(ref resp) if resp.status == 400 => Err(ClientError::Syntax(resp.to_string())),
                Ok(ref resp) => Err(ClientError::Unexpected(format!("Unexpected response. Status: {}; Body: \"{}\"", resp.status, resp.to_string()))),
                Err(reason) => Err(ClientError::Communication(reason))
            }
        }))
    }

    fn write_one(&self, measurement: Measurement, precision: Option<Precision>) -> ClientWriteResult {
        self.write_many(&[measurement], precision)
    }

    fn write_many(&self, measurements: &[Measurement], precision: Option<Precision>) -> ClientWriteResult {
        let host = self.get_host();

        let futures = measurements.chunks(self.max_batch as usize).map(|chunk| {
            let mut lines = Vec::new();

            for measurement in chunk {
                lines.push(self.serializer.serialize(measurement));
            }

            let mut query = HashMap::new();
            query.insert(String::from("db"), self.credentials.database.to_string());

            if let Some(ref precision) = precision {
                query.insert(String::from("precision"), precision.to_string());
            }

            let request = Request {
                url: format!("{}/write", host.to_string()),
                method: Method::POST,
                auth: Some(Auth {
                    username: self.credentials.username.clone(),
                    password: self.credentials.password.clone()
                }),
                query: Some(query),
                body: Some(lines.join("\n"))
            };

            self.hurl.request(request).then(|res| {
                match res {
                    Ok(ref resp) if resp.status == 204 => Ok(()),
                    Ok(ref resp) if resp.status == 200 => Err(ClientError::CouldNotComplete(resp.to_string())),
                    Ok(ref resp) if resp.status == 400 => Err(ClientError::Syntax(resp.to_string())),
                    Ok(ref resp) => Err(ClientError::Unexpected(format!("Unexpected response. Status: {}; Body: \"{}\"", resp.status, resp.to_string()))),
                    Err(reason) => Err(ClientError::Communication(reason))
                }
            })
        });

        Box::new(stream::futures_ordered(futures).for_each(|_| Ok(())))
    }
}



#[cfg(test)]
mod tests {
    use ::serializer::Serializer;
    use ::client::{Client};
    use super::HttpClient;
    use ::client::{Credentials, Precision};
    use ::hurl::{Hurl, Request, Response, HurlResult};
    use ::measurement::Measurement;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use ::futures::{self, Future};

    struct MockSerializer {
        serialize_count: AtomicUsize,
    }

    impl MockSerializer {
        fn new() -> MockSerializer {
            MockSerializer {
                serialize_count: AtomicUsize::new(0),
            }
        }
    }

    impl Serializer for MockSerializer {
        fn serialize(&self, measurement: &Measurement) -> String {
            println!("serializing: {:?}", measurement);
            self.serialize_count.fetch_add(1, Ordering::SeqCst);
            "serialized".to_string()
        }
    }

    struct MockHurl {
        request_count: AtomicUsize,
        result: Box<(Fn() -> HurlResult) + Send + Sync>
    }

    impl MockHurl {
        fn new(result: Box<(Fn() -> HurlResult) + Send + Sync>) -> MockHurl {
            MockHurl {
                request_count: AtomicUsize::new(0),
                result: result
            }
        }
    }

    impl Hurl for MockHurl {
        fn request(&self, req: Request) -> HurlResult {
            println!("sending: {:?}", req);
            self.request_count.fetch_add(1, Ordering::SeqCst);
            let ref f = self.result;
            f()
        }
    }

    fn before(result: Box<(Fn() -> HurlResult) + Send + Sync>) -> HttpClient {
        let credentials = Credentials {
            username: String::from("gobwas"),
            password: String::from("1234"),
            database: String::from("test")
        };

        let serializer = MockSerializer::new();
        let hurl = MockHurl::new(result);

        HttpClient::new(credentials, Box::new(serializer), Box::new(hurl))
    }

    #[test]
    fn test_write_one() {
        let mut client = before(Box::new(|| Box::new(futures::future::ok(Response { status: 204, body: "Ok".to_string() }))));
        client.add_host("http://localhost:8086");
        ::tokio::run(client.write_one(Measurement::new("key"), Some(Precision::Nanoseconds)).map_err(|e| panic!(e)));
    }

    #[test]
    fn test_write_many() {
        let mut client = before(Box::new(|| Box::new(futures::future::ok(Response { status: 204, body: "Ok".to_string() }))));
        client.add_host("http://localhost:8086");
        assert!(client.write_many(&[Measurement::new("key")], Some(Precision::Nanoseconds)).wait().is_ok());
    }
}



