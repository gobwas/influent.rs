extern crate influent;

use influent::create_client;
use influent::client::{Client, Credentials};
use influent::client::http::HttpClient;
use influent::measurement::{Measurement, Value};

fn before<'a>() -> HttpClient<'a> {
	let credentials = Credentials {
        username: "gobwas",
        password: "xxxx",
        database: "test"
    };

    let client = create_client(credentials, vec!["http://localhost:8086"]);

    client.query("drop database test".to_string(), None).unwrap();
    client.query("create database test".to_string(), None).unwrap();

    client
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

    assert!(client.write_one(measurement, None).is_ok());

    let fixture = "{\"results\":[{\"series\":[{\"name\":\"sut\",\"columns\":[\"time\",\"boolean\",\"float\",\"integer\",\"string\",\"tag\",\"tag, with comma\",\"with, comma\"],\"values\":[[\"2015-06-11T20:46:02Z\",false,10,10,\"string\",\"value\",\"three, four\",\"comma, with\"]]}]}]}";
    assert_eq!(fixture, client.query("select * from \"sut\"".to_string(), None).unwrap());
}
