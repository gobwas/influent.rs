use ::measurement::{Measurement, Value};
use ::serializer::Serializer;

pub struct LineSerializer;

impl LineSerializer {
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
    vec![
        "\"".to_string(),
        s.replace("\"", "\\\""),
        "\"".to_string()
    ].connect("")
}

fn as_integer(i: &i64) -> String {
    i.to_string()
}

fn as_float(f: &f64) -> String {
    let s = f.to_string();

    match s.find(".") {
        Some(_) => s,
        None => vec![s, ".0".to_string()].connect("")
    }
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

        line.connect("")
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
        assert_eq!("10", as_integer(&10i64));
    }

    #[test]
    fn test_as_float() {
        assert_eq!("10.1", as_float(&10.1f64));
        assert_eq!("10.0", as_float(&10f64));
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

        assert_eq!("key,one\\ \\,two=three\\,\\ four,tag=value b=f,f=10.0,i=10,one\\,\\ two=\"three\",s=\"string\" 10", serializer.serialize(&measurement));
    }
}




