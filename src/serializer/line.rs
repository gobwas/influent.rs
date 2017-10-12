use ::measurement::{Measurement, Value};
use ::serializer::Serializer;

pub struct LineSerializer;

/// Line spec `Measurement` serializer.
impl LineSerializer {
    /// Constructs new `LineSerializer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use influent::serializer::Serializer;
    /// use influent::serializer::line::LineSerializer;
    /// use influent::measurement::{Measurement, Value};
    ///
    /// let serializer = LineSerializer::new();
    /// let mut measurement = Measurement::new("key");
    ///
    /// measurement.add_field("field", Value::String("value"));
    /// measurement.add_tag("tag", "value");
    ///
    /// assert_eq!("key,tag=value field=\"value\"", serializer.serialize(&measurement));
    /// ```
    pub fn new() -> LineSerializer {
        LineSerializer
    }
}

fn escape(s: &str) -> String {
    s
        .replace(" ", "\\ ")
        .replace(",", "\\,")
}

fn as_string(s: &str) -> String {
    format!("\"{}\"", s.replace("\"", "\\\""))
}

fn as_integer(i: &i64) -> String {
    format!("{}i", i)
}

fn as_float(f: &f64) -> String {
    f.to_string()
}

fn as_boolean(b: &bool) -> String {
    if *b { "t".to_string() } else { "f".to_string() }
}

impl Serializer for LineSerializer {
    fn serialize(&self, measurement: &Measurement) -> String {
        let mut line = vec![escape(measurement.key)];

        for (tag, value) in measurement.tags.iter() {
            line.push(",".to_string());
            line.push(escape(tag));
            line.push("=".to_string());
            line.push(escape(value));
        }

        let mut was_spaced = false;

        for (field, value) in measurement.fields.iter() {
            line.push({if !was_spaced { was_spaced = true; " " } else { "," }}.to_string());
            line.push(escape(field));
            line.push("=".to_string());

            match value {
                &Value::String(ref s)  => line.push(as_string(s)),
                &Value::Integer(ref i) => line.push(as_integer(i)),
                &Value::Float(ref f)   => line.push(as_float(f)),
                &Value::Boolean(ref b) => line.push(as_boolean(b))
            };
        }

        match measurement.timestamp {
            Some(t) => {
                line.push(" ".to_string());
                line.push(t.to_string());
            }
            _ => {}
        }

        line.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::{as_boolean, as_string, as_integer, as_float, escape, LineSerializer};
    use ::serializer::Serializer;
    use ::measurement::{Measurement, Value};

    #[test]
    fn test_as_boolean() {
        assert_eq!("t", as_boolean(&true));
        assert_eq!("f", as_boolean(&false));
    }

    #[test]
    fn test_as_string() {
        assert_eq!("\"\\\"hello\\\"\"", as_string(&"\"hello\""));
    }

    #[test]
    fn test_as_integer() {
        assert_eq!("1i",    as_integer(&1i64));
        assert_eq!("345i",  as_integer(&345i64));
        assert_eq!("2015i", as_integer(&2015i64));
        assert_eq!("-10i",  as_integer(&-10i64));
    }

    #[test]
    fn test_as_float() {
        assert_eq!("1", as_float(&1f64));
        assert_eq!("1", as_float(&1.0f64));
        assert_eq!("-3.14", as_float(&-3.14f64));
        assert_eq!("10", as_float(&10f64));
    }

    #[test]
    fn test_escape() {
        assert_eq!("\\ ", escape(" "));
        assert_eq!("\\,", escape(","));
        assert_eq!("hello\\,\\ gobwas", escape("hello, gobwas"));
    }

    #[test]
    fn test_line_serializer() {
        let serializer = LineSerializer::new();
        let mut measurement = Measurement::new("key");

        measurement.add_field("s", Value::String("string"));
        measurement.add_field("i", Value::Integer(10));
        measurement.add_field("f", Value::Float(10f64));
        measurement.add_field("b", Value::Boolean(false));

        measurement.add_tag("tag", "value");
        
        measurement.add_field("one, two", Value::String("three"));
        measurement.add_tag("one ,two", "three, four");


        measurement.set_timestamp(10);

        assert_eq!("key,one\\ \\,two=three\\,\\ four,tag=value b=f,f=10,i=10i,one\\,\\ two=\"three\",s=\"string\" 10", serializer.serialize(&measurement));
    }

    #[test]
    fn test_line_serializer_long_timestamp() {
        let serializer = LineSerializer::new();
        let mut measurement = Measurement::new("key");

        measurement.add_field("s", Value::String("string"));

        measurement.set_timestamp(1434055562000000000);

        assert_eq!("key s=\"string\" 1434055562000000000", serializer.serialize(&measurement));
    }
}




