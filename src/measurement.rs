use std::collections::BTreeMap;

pub enum Value<'a> {
    String(&'a str),
    Float(f64),
    Integer(i64),
    Boolean(bool)
}

pub struct Measurement<'a> {
    pub key: &'a str,
    pub timestamp: Option<i32>,
    pub fields: BTreeMap<&'a str, Value<'a>>,
    pub tags: BTreeMap<&'a str, &'a str>
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