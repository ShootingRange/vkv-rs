#[macro_use]
extern crate pest_derive;

mod encode;
mod parser;

pub use encode::encode_vkv as encode;
pub use parser::parse_vkv as decode;

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Root<'a> {
    name: &'a str,
    elements: Vec<KeyValue<'a>>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct KeyValue<'a> {
    key: &'a str,
    value: Value<'a>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
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

impl<'a> From<&Root<'a>> for KeyValue<'a> {
    fn from(root: &Root<'a>) -> KeyValue<'a> {
        KeyValue {
            key: root.name,
            value: Value::Section(root.elements.clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_root() {
        let encoded = Root {
            name: "root",
            elements: vec![],
        };

        assert_eq!(encoded, decode(encode(&encoded).unwrap().as_str()).unwrap())
    }

    #[test]
    fn alt_root_name() {
        let encoded = Root {
            name: "foo",
            elements: vec![],
        };

        assert_eq!(encoded, decode(encode(&encoded).unwrap().as_str()).unwrap())
    }

    #[test]
    fn string_key() {
        let encoded = Root {
            name: "root",
            elements: vec![KeyValue {
                key: "foo",
                value: Value::String("bar"),
            }],
        };

        assert_eq!(encoded, decode(encode(&encoded).unwrap().as_str()).unwrap())
    }

    #[test]
    fn empty_sub_section() {
        let encoded = Root {
            name: "root",
            elements: vec![KeyValue {
                key: "foo",
                value: Value::Section(vec![]),
            }],
        };

        assert_eq!(encoded, decode(encode(&encoded).unwrap().as_str()).unwrap())
    }

    #[test]
    fn empty_sub_sub_section() {
        let encoded = Root {
            name: "root",
            elements: vec![KeyValue {
                key: "foo",
                value: Value::Section(vec![KeyValue {
                    key: "bar",
                    value: Value::Section(vec![]),
                }]),
            }],
        };

        assert_eq!(encoded, decode(encode(&encoded).unwrap().as_str()).unwrap())
    }

    #[test]
    fn empty_multiple_key() {
        let encoded = Root {
            name: "root",
            elements: vec![
                KeyValue {
                    key: "foo",
                    value: Value::String("bar"),
                },
                KeyValue {
                    key: "foo",
                    value: Value::Section(vec![]),
                },
                KeyValue {
                    key: "baz",
                    value: Value::String("foz"),
                },
            ],
        };

        assert_eq!(encoded, decode(encode(&encoded).unwrap().as_str()).unwrap())
    }
}
