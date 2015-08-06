use ::measurement::Measurement;

pub mod line;

pub trait Serializer {
    fn serialize(&self, measurement: &Measurement) -> String;
}