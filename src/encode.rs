use crate::{KeyValue, Root, Value};
use std::fmt::Write;

pub fn encode_vkv(root: &Root) -> Result<String, Box<dyn std::error::Error>> {
    let mut encoded = String::new();

    encode_value(KeyValue::from(root), 0, &mut encoded)?;

    Ok(encoded)
}

fn encode_values(
    kvs: Vec<KeyValue>,
    indent: usize,
    write_to: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    for kv in kvs {
        encode_value(kv, indent, write_to)?;
    }

    Ok(())
}

fn encode_value(
    kv: KeyValue,
    indent: usize,
    write_to: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    match kv.value {
        Value::Section(values) => {
            write!(
                write_to,
                "{}\"{}\"\n{}{{\n",
                "    ".repeat(indent),
                encode_string(kv.key),
                "    ".repeat(indent)
            )?;

            encode_values(values, indent + 1, write_to)?;

            write!(write_to, "{}}}\n", "    ".repeat(indent))?;
        }
        Value::String(value) => {
            write!(
                write_to,
                "{}\"{}\" \"{}\"\n",
                "    ".repeat(indent),
                encode_string(kv.key),
                encode_string(value)
            )?;
        }
    }

    Ok(())
}

fn encode_string(text: &str) -> String {
    // TODO escaping of special character in strings
    text.to_string()
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

        let decoded = r#""root"
{
}
"#;

        assert_eq!(encode_vkv(&encoded).unwrap(), decoded);
    }

    #[test]
    fn alt_root_name() {
        let encoded = Root {
            name: "foo",
            elements: vec![],
        };

        let decoded = r#""foo"
{
}
"#;

        assert_eq!(encode_vkv(&encoded).unwrap(), decoded);
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

        let decoded = r#""root"
{
    "foo" "bar"
}
"#;

        assert_eq!(encode_vkv(&encoded).unwrap(), decoded);
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

        let decoded = r#""root"
{
    "foo"
    {
    }
}
"#;

        assert_eq!(encode_vkv(&encoded).unwrap(), decoded);
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

        let decoded = r#""root"
{
    "foo"
    {
        "bar"
        {
        }
    }
}
"#;

        assert_eq!(encode_vkv(&encoded).unwrap(), decoded);
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

        let decoded = r#""root"
{
    "foo" "bar"
    "foo"
    {
    }
    "baz" "foz"
}
"#;

        assert_eq!(encode_vkv(&encoded).unwrap(), decoded);
    }
}
