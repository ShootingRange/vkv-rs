#[macro_use]
extern crate pest_derive;

mod parser;

use parser::{Rule, VKVParser};
use pest::iterators::Pair;
use pest::{Parser, RuleType};
use std::error::Error;

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

pub fn parse_vkv(vkv: &str) -> Result<Root, Box<dyn Error>> {
    let start_pair: Pair<Rule> = VKVParser::parse(Rule::start, vkv)?.next().unwrap();
    debug_assert_eq!(start_pair.as_rule(), Rule::start);

    let section = start_pair
        .into_inner()
        .next()
        .expect("Start was empty, it should have contained a section");
    Ok(parse_root_section(section))
}

fn parse_root_section(pair: Pair<Rule>) -> Root {
    debug_assert_eq!(pair.as_rule(), Rule::root_section);

    let mut inner = pair.into_inner();
    let pair_name = inner
        .next()
        .expect("Expected a first element in root_section");
    let pair_body = inner
        .next()
        .expect("Expected a second element in root_section");

    Root {
        name: parse_string(pair_name),
        elements: parse_section_body(pair_body),
    }
}

fn parse_section_body(pair: Pair<Rule>) -> Vec<KeyValue> {
    debug_assert_eq!(pair.as_rule(), Rule::section_body);

    pair.into_inner()
        .filter(|inner_pair| inner_pair.as_rule() == Rule::element)
        .map(|element_pair| parse_element(element_pair))
        .collect()
}

fn parse_element(pair: Pair<Rule>) -> KeyValue {
    debug_assert_eq!(pair.as_rule(), Rule::element);

    let mut inner = pair.into_inner();

    // Skip whitespaces
    loop {
        if let Some(inner_pair) = inner.peek() {
            if inner_pair.as_rule() == Rule::ws {
                inner.next();
            } else {
                break;
            }
        }
    }

    let pair_key = inner
        .next()
        .expect("Expected a first pair for the element's key");
    let pair_value = inner
        .next()
        .expect("Expected a second pair for the element's value");

    KeyValue {
        key: parse_key(pair_key),
        value: parse_value(pair_value),
    }
}

fn parse_key(pair: Pair<Rule>) -> &str {
    debug_assert_eq!(pair.as_rule(), Rule::key);

    parse_string(
        pair.into_inner()
            .next()
            .expect("Expected a pair for the string in a key"),
    )
}

fn parse_value(pair: Pair<Rule>) -> Value {
    let value_inner = pair
        .into_inner()
        .next()
        .expect("Expected a sub value in value pair");
    match value_inner.as_rule() {
        Rule::value_simple => parse_value_simple(value_inner),
        Rule::value_section => parse_value_section(value_inner),
        _ => unreachable!(),
    }
}

fn parse_value_simple(pair: Pair<Rule>) -> Value {
    debug_assert_eq!(pair.as_rule(), Rule::value_simple);

    let mut inner = pair.into_inner();

    // Skip indent
    loop {
        if let Some(inner_pair) = inner.peek() {
            if inner_pair.as_rule() == Rule::indent {
                inner.next();
            } else {
                break;
            }
        }
    }

    // assumes a string for now
    let string_pair = inner
        .next()
        .expect("Expected value_simple to contain a string");
    Value::String(parse_string(string_pair))
}

fn parse_value_section(pair: Pair<Rule>) -> Value {
    debug_assert_eq!(pair.as_rule(), Rule::value_section);

    let string_pair = pair
        .into_inner()
        .next()
        .expect("Expected value_section to contain a string");
    Value::Section(parse_section_body(string_pair))
}

fn parse_string<'a>(pair: Pair<'a, Rule>) -> &'a str {
    debug_assert_eq!(pair.as_rule(), Rule::string);

    pair.into_inner()
        .next()
        .expect("Missing inner string")
        .as_str()
}

impl<'a> From<Root<'a>> for KeyValue<'a> {
    fn from(root: Root<'a>) -> KeyValue<'a> {
        KeyValue {
            key: root.name,
            value: Value::Section(root.elements),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{parse_vkv, KeyValue, Root, Value};

    #[test]
    fn parse_vkv_1() {
        let text = "  \"f\"\n{\n}";
        let root = parse_vkv(text).unwrap();
        assert_eq!(
            root,
            Root {
                name: "f",
                elements: vec![],
            }
        )
    }

    #[test]
    fn parse_vkv_2() {
        let text = r#""root"
{
    "foo" "bar"
}
"#;

        let root = parse_vkv(text).unwrap();
        assert_eq!(
            root,
            Root {
                name: "root",
                elements: vec![KeyValue {
                    key: "foo",
                    value: Value::String("bar")
                }],
            }
        )
    }

    #[test]
    fn parse_vkv_3() {
        let text = r#""root"
{
    "foo"
    {}
}
"#;

        let root = parse_vkv(text).unwrap();
        assert_eq!(
            root,
            Root {
                name: "root",
                elements: vec![KeyValue {
                    key: "foo",
                    value: Value::Section(vec![])
                }],
            }
        )
    }

    #[test]
    fn parse_vkv_4() {
        let text = r#""root"
{
    "foo"
    {
        "bar" "baz"
    }
}
"#;

        let root = parse_vkv(text).unwrap();
        assert_eq!(
            root,
            Root {
                name: "root",
                elements: vec![KeyValue {
                    key: "foo",
                    value: Value::Section(vec![KeyValue {
                        key: "bar",
                        value: Value::String("baz"),
                    }]),
                }],
            }
        )
    }

    #[test]
    fn parse_vkv_5() {
        let text = r#""root"
{
    "bar" "baz"
    "foo"
    {
    }
}
"#;

        let root = parse_vkv(text).unwrap();
        assert_eq!(
            root,
            Root {
                name: "root",
                elements: vec![
                    KeyValue {
                        key: "bar",
                        value: Value::String("baz"),
                    },
                    KeyValue {
                        key: "foo",
                        value: Value::Section(vec![]),
                    }
                ],
            }
        )
    }
}
