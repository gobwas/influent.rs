use std::collections::BTreeMap;

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
    pub timestamp: Option<i32>,

    /// Map of fields.
    pub fields: BTreeMap<&'a str, Value<'a>>,
    
    /// Map of tags.
    pub tags: BTreeMap<&'a str, &'a str>
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
    pub fn add_field(&mut self, field: &'a str, value: Value<'a>) {
        self.fields.insert(field, value);
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
    pub fn add_tag(&mut self, tag: &'a str, value: &'a str) {
        self.tags.insert(tag, value);
    }

    /// Sets the timestamp of the measurement.
    ///
    /// # Examples
    ///
    /// ```
    /// use influent::measurement::{Measurement, Value};
    ///
    /// let mut measurement = Measurement::new("key");
    ///
    /// measurement.set_timestamp(1440924047129)
    /// ```
    pub fn set_timestamp(&mut self, timestamp: i32) {
        self.timestamp = Some(timestamp);
    }
}