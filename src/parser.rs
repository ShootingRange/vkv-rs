use pest::Parser;

#[derive(Parser)]
#[grammar = "vkv.pest"]
pub(crate) struct VKVParser {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn section_empty() {
        let text = "\"foo\"\n{}";

        VKVParser::parse(Rule::start, text).unwrap();
    }

    #[test]
    fn section_empty_newline() {
        let text = "\"foo\"\n{\n}";

        VKVParser::parse(Rule::start, text).unwrap();
    }

    #[test]
    fn section_sub_section() {
        let text = r#""foo"
{
    "bar"
    {
    }
}"#;

        VKVParser::parse(Rule::start, text).unwrap();
    }

    #[test]
    fn section_indented() {
        let text = r#"    "foo"
    {
    }"#;

        VKVParser::parse(Rule::start, text).unwrap();
    }

    #[test]
    fn key_string() {
        let text = r#"    "foo"
    {
        "bar" "baz"
    }"#;

        VKVParser::parse(Rule::start, text).unwrap();
    }

    #[test]
    fn section_mutiple_children() {
        let text = r#"    "foo"
    {
        "bar" "baz"
        "nar" "nal"
        "jok"
        {
            "adsf" "jofjfo"
        }
        "adodo" "fofo"
    }"#;

        VKVParser::parse(Rule::start, text).unwrap();
    }

    #[test]
    fn indent_space() {
        let text = " ";

        VKVParser::parse(Rule::indent, text).unwrap();
    }

    #[test]
    fn indent_tab() {
        let text = "\t";

        VKVParser::parse(Rule::indent, text).unwrap();
    }

    #[test]
    fn string_1() {
        let text = "\"\"";

        VKVParser::parse(Rule::string, text).unwrap();
    }

    #[test]
    fn string_2() {
        let text = "\"foo\"";

        VKVParser::parse(Rule::string, text).unwrap();
    }

    #[test]
    fn string_3() {
        let text = "\"foo bar\"";

        let mut parsed: pest::iterators::Pairs<'_, Rule> =
            VKVParser::parse(Rule::string, text).unwrap();
        let pair = parsed
            .next()
            .expect("Expected something to have been parsed");
        assert!(parsed.next().is_none(), "only expected a single pair");
        let span = pair.as_span();
        assert_eq!(span.start(), 0, "did not start at the first character");
        assert_eq!(
            span.end(),
            text.len(),
            "did not parse the expected amount of characters"
        );
    }

    #[test]
    fn string_4() {
        // This should end after the second quote character
        let text = "\"foo\"bar\"";

        let mut parsed: pest::iterators::Pairs<'_, Rule> =
            VKVParser::parse(Rule::string, text).unwrap();
        let pair = parsed
            .next()
            .expect("Expected something to have been parsed");
        assert!(parsed.next().is_none(), "only expected a single pair");
        let span = pair.as_span();
        assert_eq!(span.start(), 0, "did not start at the first character");
        assert_eq!(
            span.end(),
            5,
            "did not parse the expected amount of characters"
        );
    }

    #[test]
    fn empty() {
        // Input must start with a section
        let text = "  ";

        assert!(VKVParser::parse(Rule::string, text).is_err());
    }
}
