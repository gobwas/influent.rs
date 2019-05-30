extern crate influent;
extern crate tokio;
extern crate futures;

use influent::create_client;
use influent::client::{Client, Credentials};
use influent::client::http::HttpClient;
use influent::measurement::{Measurement, Value};
use futures::Future;

use std::sync::Arc;

fn before() -> HttpClient {
    let credentials = Credentials {
        username: String::from("gobwas"),
        password: String::from("xxxx"),
        database: String::from("test"),
    };

    let client = Arc::new(create_client(
        credentials,
        vec![String::from("http://localhost:8086")],
    ));

    {
        let client = client.clone();
        let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();
        rt.block_on(
            client
                .query("drop database test".to_string(), None)
                .then(move |_| client.query("create database test".to_string(), None))
                .map(|_| ())
                .map_err(|_| ()),
        )
        .unwrap();
    }

    if let Ok(client) = Arc::try_unwrap(client) {
        return client;
    }
    panic!("wtf")
}

#[test]
fn test_write_measurement() {
    let client = before();

    let mut measurement = Measurement::new("sut");

    measurement.add_field("string", Value::String("string"));
    measurement.add_field("integer", Value::Integer(10));
    measurement.add_field("float", Value::Float(10f64));
    measurement.add_field("boolean", Value::Boolean(false));
    measurement.add_field("with, comma", Value::String("comma, with"));

    measurement.add_tag("tag", "value");
    measurement.add_tag("tag, with comma", "three, four");

    measurement.set_timestamp(1_434_055_562_000_000_000);

    let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();

    rt.block_on(client.write_one(measurement, None).then(move |_| {
        client.query("select * from \"sut\"".to_string(), None)
    }).map(|res| {
        let fixture = "{\"results\":[{\"statement_id\":0,\"series\":[{\"name\":\"sut\",\"columns\":[\"time\",\"boolean\",\"float\",\"integer\",\"string\",\"tag\",\"tag, with comma\",\"with, comma\"],\"values\":[[\"2015-06-11T20:46:02Z\",false,10,10,\"string\",\"value\",\"three, four\",\"comma, with\"]]}]}]}\n";
        assert_eq!(fixture, res);
    }).map_err(|e| println!("{:?}", e))).unwrap();
}
