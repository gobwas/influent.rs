use ::hyper::Client as HyperClient;
use ::hyper::header::Connection;
use ::measurement::Measurement;
use ::serializer::Serializer;
use ::client::{Client, Credentials};
use ::std::io::Read;

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