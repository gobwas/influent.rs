use async_trait::async_trait;
use crate::measurement::Measurement;
use crate::serializer::Serializer;
use crate::client::{Precision, Client, Credentials, ClientError};
use crate::hurl::{Hurl, Request, Method, Auth};
use std::collections::HashMap;

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

pub struct HttpClient<'a> {
    credentials: Credentials<'a>,
    serializer: Box<dyn Serializer + Send + Sync>,
    hurl: Box<dyn Hurl + Send + Sync>,
    hosts: Vec<&'a str>,
    pub max_batch: u16
}

impl<'a> HttpClient<'a> {
    pub fn new(credentials: Credentials<'a>, serializer: Box<dyn Serializer + Send + Sync>, hurl: Box<dyn Hurl + Send + Sync>) -> HttpClient<'a> {
        HttpClient {
            credentials,
            serializer,
            hurl,
            hosts: vec![],
            max_batch: MAX_BATCH
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

#[async_trait]
impl<'a> Client for HttpClient<'a> {
    async fn query(&self, q: String, epoch: Option<Precision>) -> Result<String, ClientError> {
        let host = self.get_host();

        let mut query = HashMap::new();
        query.insert("db", self.credentials.database.to_string());
        query.insert("q", q);

        if let Some(ref epoch) = epoch {
            query.insert("epoch", epoch.to_string());
        }

        let auth = if self.credentials.username == "" && self.credentials.password == "" {
            None
        } else {
            Some(Auth {
                username: self.credentials.username,
                password: self.credentials.password
            })
        };

        let request = Request {
            url: &*{host.to_string() + "/query"},
            method: Method::GET,
            auth,
            query: Some(query),
            body: None
        };

        let resp = self.hurl.request(request).await
            .map_err(ClientError::Communication)?;
        match resp.status {
            200 => Ok(resp.to_string()),
            400 => Err(ClientError::Syntax(resp.to_string())),
            _ => Err(ClientError::Unexpected(format!("Unexpected response. Status: {}; Body: \"{}\"", resp.status, resp.to_string()))),
        }
    }

    async fn write_one(&self, measurement: Measurement<'_>, precision: Option<Precision>) -> Result<(), ClientError> {
        self.write_many(&[measurement], precision).await
    }

    async fn write_many(&self, measurements: &[Measurement<'_>], precision: Option<Precision>) -> Result<(), ClientError> {
        let host = self.get_host();

        for chunk in measurements.chunks(self.max_batch as usize) {
            let mut lines = Vec::new();

            for measurement in chunk {
                lines.push(self.serializer.serialize(measurement));
            }

            let mut query = HashMap::new();
            query.insert("db", self.credentials.database.to_string());

            if let Some(ref precision) = precision {
                query.insert("precision", precision.to_string());
            }

            let request = Request {
                url: &*{host.to_string() + "/write"},
                method: Method::POST,
                auth: Some(Auth {
                    username: self.credentials.username,
                    password: self.credentials.password
                }),
                query: Some(query),
                body: Some(lines.join("\n"))
            };

            let resp = self.hurl.request(request).await
                .map_err(ClientError::Communication)?;
            match resp.status {
                204 => (),
                200 => return Err(ClientError::CouldNotComplete(resp.to_string())),
                400 => return Err(ClientError::Syntax(resp.to_string())),
                _ => return Err(ClientError::Unexpected(format!("Unexpected response. Status: {}; Body: \"{}\"", resp.status, resp.to_string())))
            };
        }

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use crate::serializer::Serializer;
    use crate::client::{Client};
    use super::HttpClient;
    use crate::client::{Credentials, Precision};
    use crate::hurl::{Hurl, Request, Response};
    use crate::measurement::Measurement;
    use std::sync::atomic::{AtomicUsize, Ordering};

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
        result: Box<dyn (Fn() -> Result<Response, String>) + Send + Sync>
    }

    impl MockHurl {
        fn new(result: Box<dyn (Fn() -> Result<Response, String>) + Send + Sync>) -> MockHurl {
            MockHurl {
                request_count: AtomicUsize::new(0),
                result
            }
        }
    }

    #[async_trait]
    impl Hurl for MockHurl {
        async fn request(&self, req: Request<'_>) -> Result<Response, String> {
            println!("sending: {:?}", req);
            self.request_count.fetch_add(1, Ordering::SeqCst);
            let ref f = self.result;
            f()
        }
    }

    fn before<'a>(result: Box<dyn (Fn() -> Result<Response, String>) + Send + Sync>) -> HttpClient<'a> {
        let credentials = Credentials {
            username: "gobwas",
            password: "1234",
            database: "test"
        };

        let serializer = MockSerializer::new();
        let hurl = MockHurl::new(result);

        HttpClient::new(credentials, Box::new(serializer), Box::new(hurl))
    }

    #[tokio::test]
    async fn test_write_one() {
        let mut client = before(Box::new(|| Ok(Response { status: 204, body: "Ok".to_string() })));
        client.add_host("http://localhost:8086");
        if let Err(e) = client.write_one(Measurement::new("key"), Some(Precision::Nanoseconds)).await {
            panic!(e);
        }
    }

    #[tokio::test]
    async fn test_write_many() {
        let mut client = before(Box::new(|| Ok(Response { status: 204, body: "Ok".to_string() })));
        client.add_host("http://localhost:8086");
        assert!(client.write_many(&[Measurement::new("key")], Some(Precision::Nanoseconds)).await.is_ok());
    }
}



