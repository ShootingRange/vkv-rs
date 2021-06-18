#[macro_use]
extern crate pest_derive;

mod encode;
mod parser;

pub use encode::encode_vkv as encode;
pub use parser::parse_vkv as decode;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Root<'a> {
    name: &'a str,
    elements: Vec<KeyValue<'a>>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct KeyValue<'a> {
    key: &'a str,
    value: Value<'a>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Value<'a> {
    Section(Vec<KeyValue<'a>>),
    String(&'a str),
}

impl<'a> From<Root<'a>> for KeyValue<'a> {
    fn from(root: Root<'a>) -> KeyValue<'a> {
        KeyValue {
            key: root.name,
            value: Value::Section(root.elements),
        }
    }
}
