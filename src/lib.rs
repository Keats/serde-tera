extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate indexmap;

mod error;
mod value;


pub use error::{Error, Result};
pub use value::{to_value, Value, Map};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[test]
fn can_convert_enum_members() {
    let val = Animal::Frog("Froggy".to_string(), vec![1, 2, 3]);
    let res = to_value(val).unwrap();
    println!("{:?}", res);
    assert!(false);
}
