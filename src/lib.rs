#[macro_use]
extern crate pest_derive;

mod parser;

pub use parser::parse_vkv as parse;

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
