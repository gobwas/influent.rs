use ::measurement::Measurement;
use ::serializer::Serializer;
use ::client::{Precision, Client, Credentials, ClientError, ClientReadResult, ClientWriteResult};
use std::net::{UdpSocket, ToSocketAddrs};

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

pub struct UdpClient<'a> {
    serializer: Box<Serializer>,
    hosts: Vec<&'a str>,
    pub max_batch: u16
}

impl<'a> UdpClient<'a> {
    pub fn new(serializer: Box<Serializer>) -> Self {
        UdpClient {
            serializer: serializer,
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

impl<'a> Client for UdpClient<'a> {
    fn query(&self, _: String, _: Option<Precision>) -> ClientReadResult {
        Err(ClientError::CouldNotComplete("querying is not supported over UDP".to_string()))
    }

    fn write_one(&self, measurement: Measurement, precision: Option<Precision>) -> ClientWriteResult {
        self.write_many(&[measurement], precision)
    }

    fn write_many(&self, measurements: &[Measurement], _: Option<Precision>) -> ClientWriteResult {
        let socket = try!(UdpSocket::bind("0.0.0.0:0"));
        let addr = try!(self.get_host().to_socket_addrs()).last().unwrap();

        for chunk in measurements.chunks(self.max_batch as usize) {
            let mut bytes = Vec::new();
            const MAX_UDP_PACKET_LEN: usize = 65535;

            for measurement in chunk {
                let line = self.serializer.serialize(measurement);
                let line = line.as_bytes();
                if line.len() + bytes.len() < MAX_UDP_PACKET_LEN {
                    bytes.extend_from_slice(&line[..]);
                    bytes.push(b'\n');
                } else {
                    try!(socket.send_to(&bytes[..], addr));
                    bytes.clear();
                    bytes.extend_from_slice(&line[..]);
                    bytes.push(b'\n');
                }
            }
            if !bytes.is_empty() {
                try!(socket.send_to(&bytes[..], addr));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ::serializer::line::LineSerializer;
    use ::client::{Client};
    use super::UdpClient;
    use ::client::Precision;
    use ::measurement::{Measurement,self};

    #[test]
    fn test_write_one() {
        let mut client = UdpClient::new(Box::new(LineSerializer::new()));
        client.add_host("127.0.0.1:8089");
        let mut m = Measurement::new("lol");
        let val = measurement::Value::Integer(1488);
        m.add_field("value", val);
        client.write_one(m, Some(Precision::Nanoseconds));
    }

    #[test]
    fn test_write_many() {
        let mut client = UdpClient::new(Box::new(LineSerializer::new()));
        client.add_host("127.0.0.1:8089");
        client.write_many(&[Measurement::new("kek")], Some(Precision::Nanoseconds));
    }
}
