use indexmap::IndexMap;
use serde::ser::Serialize;

mod ser;
mod from;

use error::Error;
use self::ser::Serializer;


pub type Map = IndexMap<String, Value>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    // TODO: use Cow here to allow references?
    String(String),
    Array(Vec<Value>),
    Object(Map),
}

pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(Serializer)
}
