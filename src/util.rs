use crate::model::Model;
use rusqlite::{Result as SqliteResult, Row};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, SeqAccess, IntoDeserializer};
use serde_json::Value;
use std::fmt::Display;

/// Helper function to convert a Row to a Model instance
/// Uses a custom deserializer to handle type mismatches (e.g. TEXT -> Integer)
pub fn row_to_model<T: Model>(row: &Row, fields: &[&str]) -> SqliteResult<T> {
    let mut json_map = serde_json::Map::new();

    for (idx, field) in fields.iter().enumerate() {
        // Try to get the value as different types
        let value: serde_json::Value = if let Ok(v) = row.get::<_, i64>(idx) {
            serde_json::Value::Number(v.into())
        } else if let Ok(v) = row.get::<_, String>(idx) {
            serde_json::Value::String(v)
        } else if let Ok(v) = row.get::<_, f64>(idx) {
            serde_json::Value::Number(
                serde_json::Number::from_f64(v).unwrap_or_else(|| 0.into())
            )
        } else {
            serde_json::Value::Null
        };

        json_map.insert(field.to_string(), value);
    }

    let value = Value::Object(json_map);

    // Use custom deserializer
    let loose_value = LooseValue(value);
    T::deserialize(loose_value).map_err(|e| rusqlite::Error::FromSqlConversionFailure(
        0,
        rusqlite::types::Type::Text,
        Box::new(e)
    ))
}

#[derive(Debug)]
pub struct DeserError(String);

impl Display for DeserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for DeserError {}

impl de::Error for DeserError {
    fn custom<T: Display>(msg: T) -> Self {
        DeserError(msg.to_string())
    }
}

pub struct LooseValue(pub Value);

macro_rules! impl_int_deser {
    ($name:ident, $visit:ident, $type:ty, $as_method:ident) => {
        fn $name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de> {
            match self.0 {
                Value::String(ref s) => {
                    if let Ok(n) = s.parse::<$type>() {
                        return visitor.$visit(n);
                    }
                }
                Value::Number(ref n) => {
                     if let Some(i) = n.$as_method() {
                         return visitor.$visit(i as $type);
                     }
                }
                _ => {}
            }
            self.deserialize_any(visitor)
        }
    }
}

macro_rules! impl_float_deser {
    ($name:ident, $visit:ident, $type:ty, $as_method:ident) => {
        fn $name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de> {
            match self.0 {
                Value::String(ref s) => {
                    if let Ok(n) = s.parse::<$type>() {
                        return visitor.$visit(n);
                    }
                }
                Value::Number(ref n) => {
                     if let Some(i) = n.$as_method() {
                         return visitor.$visit(i as $type);
                     }
                }
                _ => {}
            }
            self.deserialize_any(visitor)
        }
    }
}

impl<'de> Deserializer<'de> for LooseValue {
    type Error = DeserError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        match self.0 {
            Value::Null => visitor.visit_unit(),
            Value::Bool(b) => visitor.visit_bool(b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    visitor.visit_i64(i)
                } else if let Some(u) = n.as_u64() {
                    visitor.visit_u64(u)
                } else if let Some(f) = n.as_f64() {
                    visitor.visit_f64(f)
                } else {
                    Err(de::Error::custom("invalid number"))
                }
            },
            Value::String(s) => visitor.visit_string(s),
            Value::Array(a) => visitor.visit_seq(LooseSeqAccess { iter: a.into_iter() }),
            Value::Object(o) => visitor.visit_map(LooseMapAccess { iter: o.into_iter(), value: None }),
        }
    }

    impl_int_deser!(deserialize_i8, visit_i8, i8, as_i64);
    impl_int_deser!(deserialize_i16, visit_i16, i16, as_i64);
    impl_int_deser!(deserialize_i32, visit_i32, i32, as_i64);
    impl_int_deser!(deserialize_i64, visit_i64, i64, as_i64);

    impl_int_deser!(deserialize_u8, visit_u8, u8, as_u64);
    impl_int_deser!(deserialize_u16, visit_u16, u16, as_u64);
    impl_int_deser!(deserialize_u32, visit_u32, u32, as_u64);
    impl_int_deser!(deserialize_u64, visit_u64, u64, as_u64);

    impl_float_deser!(deserialize_f32, visit_f32, f32, as_f64);
    impl_float_deser!(deserialize_f64, visit_f64, f64, as_f64);

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        match self.0 {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
         if let Value::String(s) = self.0 {
             visitor.visit_enum(s.into_deserializer())
         } else {
             self.deserialize_any(visitor)
         }
    }

    serde::forward_to_deserialize_any! {
        bool char str string bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

struct LooseSeqAccess {
    iter: std::vec::IntoIter<Value>,
}

impl<'de> SeqAccess<'de> for LooseSeqAccess {
    type Error = DeserError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where T: de::DeserializeSeed<'de> {
        match self.iter.next() {
            Some(value) => seed.deserialize(LooseValue(value)).map(Some),
            None => Ok(None),
        }
    }
}

struct LooseMapAccess {
    iter: serde_json::map::IntoIter,
    value: Option<Value>,
}

impl<'de> MapAccess<'de> for LooseMapAccess {
    type Error = DeserError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where K: de::DeserializeSeed<'de> {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key.into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where V: de::DeserializeSeed<'de> {
        match self.value.take() {
            Some(value) => seed.deserialize(LooseValue(value)),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}
