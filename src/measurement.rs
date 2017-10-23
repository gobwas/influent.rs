use std::collections::BTreeMap;
use std::borrow::Cow;
#[derive(Debug)]
/// Measurement's field value.
pub enum Value<'a> {
    /// String.
    String(&'a str),
    /// Floating point number.
    Float(f64),
    /// Integer number.
    Integer(i64),
    /// Boolean value.
    Boolean(bool)
}

/// Measurement model.
#[derive(Debug)]
pub struct Measurement<'a> {
    /// Key.
    pub key: &'a str,

    /// Timestamp.
    pub timestamp: Option<i64>,

    /// Map of fields.
    pub fields: BTreeMap<Cow<'a, str>, Value<'a>>,
    
    /// Map of tags.
    pub tags: BTreeMap<Cow<'a,str>, Cow<'a,str>>
}

impl<'a> Measurement<'a> {
    /// Constructs a new `Measurement`.
    ///
    /// # Examples
    /// 
    /// ```
    /// use influent::measurement::Measurement;
    ///
    /// let measurement = Measurement::new("key");
    /// ```
    pub fn new(key: &str) -> Measurement {
        Measurement {
            key: key,
            timestamp: None,
            fields: BTreeMap::new(),
            tags: BTreeMap::new()
        }
    }

    /// Adds field to the measurement.
    ///
    /// # Examples
    ///
    /// ```
    /// use influent::measurement::{Measurement, Value};
    ///
    /// let mut measurement = Measurement::new("key");
    ///
    /// measurement.add_field("field", Value::String("hello"));
    /// ```
    pub fn add_field<T>(&mut self, field: T, value: Value<'a>) where T: Into<Cow<'a, str>> {
        self.fields.insert(field.into(), value);
    }

    /// Adds tag to the measurement.
    ///
    /// # Examples
    ///
    /// ```
    /// use influent::measurement::{Measurement, Value};
    ///
    /// let mut measurement = Measurement::new("key");
    ///
    /// measurement.add_tag("tag", "value");
    /// ```
    pub fn add_tag<I, K>(&mut self, tag: I, value: K) where I: Into<Cow<'a,str>>, K: Into<Cow<'a, str>> {
        self.tags.insert(tag.into(), value.into());
    }

    /// Sets the timestamp of the measurement. It should be unix timestamp in nanosecond
    ///
    /// # Examples
    ///
    /// ```
    /// use influent::measurement::{Measurement, Value};
    ///
    /// let mut measurement = Measurement::new("key");
    ///
    /// measurement.set_timestamp(1434055562000000000)
    /// ```
    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = Some(timestamp);
    }
}