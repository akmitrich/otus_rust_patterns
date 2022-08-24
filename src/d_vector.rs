use std::fmt;

use serde::{ser::{Serialize, SerializeSeq, Serializer}, de::{self, Visitor, SeqAccess}, Deserialize};
type Real = f64;

#[derive(Debug, Clone, PartialEq)]
pub struct DVector<const D: usize> {
    components: [Real; D],
}

impl<const D: usize> Default for DVector<D> {
    fn default() -> Self {
        Self {
            components: [0.; D],
        }
    }
}

impl<const D: usize> From<&[Real; D]> for DVector<D> {
    fn from(data: &[Real; D]) -> Self {
        Self::from(*data)
    }
}

impl<const D: usize> From<[Real; D]> for DVector<D> {
    fn from(components: [Real; D]) -> Self {
        Self { components }
    }
}

impl<const D: usize> Serialize for DVector<D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(D))?;
        for e in self.components.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct DVectorVisitor<const D: usize>;

impl<'de, const D: usize> Visitor<'de> for DVectorVisitor<D> {
    type Value = DVector<D>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let expect = format!("an array of {} floats", D);
        formatter.write_str(&expect)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>, {
        let mut components = [0 as Real; D];
        for c in components.iter_mut() {
            let val: Option<Real> = seq.next_element()?;
            if let Some(component) = val {
                *c = component;
            } else {
                return Err(de::Error::invalid_length(D, &self));
            }
        }
        Ok(DVector::from(components))
    }
}

impl<'de, const D: usize> Deserialize<'de> for DVector<D> {
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: serde::Deserializer<'de> {
        deserializer.deserialize_seq(DVectorVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let v = DVector::from([0., 1., 2.]);
        assert_eq!("[0.0,1.0,2.0]", serde_json::to_string(&v).unwrap());
        let x: DVector<3> = serde_json::from_str("[0.0,1.0,2.0]").unwrap();
        assert_eq!([0.0, 1.0, 2.0], x.components);
    }
}