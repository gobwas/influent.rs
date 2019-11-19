use crate::measurement::Measurement;

pub mod line;

/// `Measurement` serializer.
pub trait Serializer {
    /// Serializes measurement to String.
    fn serialize(&self, measurement: &Measurement) -> String;
}
