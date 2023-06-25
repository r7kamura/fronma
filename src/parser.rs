use crate::engines::{Engine, Yaml};
use crate::error::Error;
use crate::result::Result;

#[cfg(feature = "default")]
pub fn parse<T>(text: &str) -> Result<ParsedData<T>>
where
    T: serde::de::DeserializeOwned,
{
    parse_with_engine::<T, Yaml>(text)
}

pub fn parse_with_engine<T, E>(text: &str) -> Result<ParsedData<T>>
where
    T: serde::de::DeserializeOwned,
    E: Engine<T>,
{
    let line_pattern_lf = "---\n";
    let line_pattern_crlf = "---\r\n";

    if let Some(slice) = text.strip_prefix(line_pattern_lf) {
        let index_of_ending_line = slice
            .find(line_pattern_lf)
            .ok_or(Error::MissingEndingLine)?;
        let headers = E::parse(&slice[..index_of_ending_line])?;
        let body = &slice[(index_of_ending_line + line_pattern_lf.len())..];
        Ok(ParsedData { body, headers })
    } else if let Some(slice) = text.strip_prefix(line_pattern_crlf) {
        let index_of_ending_line = slice
            .find(line_pattern_crlf)
            .ok_or(Error::MissingEndingLine)?;
        let headers = E::parse(&slice[..index_of_ending_line])?;
        let body = &slice[(index_of_ending_line + line_pattern_crlf.len())..];
        Ok(ParsedData { body, headers })
    } else {
        Err(Error::MissingBeginningLine)
    }
}

#[derive(Debug)]
pub struct ParsedData<'a, T: serde::de::DeserializeOwned> {
    pub body: &'a str,
    pub headers: T,
}

#[cfg(feature = "default")]
#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::parser::parse;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Headers {
        title: String,
    }

    #[test]
    fn parse_with_missing_beginning_line() {
        let text = "";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::MissingBeginningLine)));
    }

    #[test]
    fn parse_with_missing_ending_line() {
        let text = "---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::MissingEndingLine)));
    }

    #[test]
    fn parse_with_missing_ending_line_crlf() {
        let text = "---\r\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::MissingEndingLine)));
    }

    #[test]
    fn parse_with_empty_frontmatter() {
        let text = "---\n---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::SerdeYaml(_))));
    }

    #[test]
    fn parse_with_empty_frontmatter_crlf() {
        let text = "---\r\n---\r\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::SerdeYaml(_))));
    }

    #[test]
    fn parse_with_missing_known_field() {
        let text = "---\ndate: 2000-01-01\n---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::SerdeYaml(_))));
    }

    #[test]
    fn parse_with_missing_known_field_crlf() {
        let text = "---\r\ndate: 2000-01-01\r\n---\r\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::SerdeYaml(_))));
    }

    #[test]
    fn parse_with_unknown_field() {
        let text = "---\ndate: 2000-01-01\ntitle: dummy_title\n---\n";
        let result = parse::<Headers>(text);
        dbg!(&result);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn parse_with_unknown_field_crlf() {
        let text = "---\r\ndate: 2000-01-01\r\ntitle: dummy_title\r\n---\r\n";
        let result = parse::<Headers>(text);
        dbg!(&result);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn parse_with_empty_known_field() {
        let text = "---\ntitle:\n---\n";
        let result = parse::<Headers>(text).unwrap();
        assert_eq!(result.headers.title, "~");
        assert_eq!(result.body, "");
    }

    #[test]
    fn parse_with_empty_known_field_crlf() {
        let text = "---\r\ntitle:\r\n---\r\n";
        let result = parse::<Headers>(text).unwrap();
        assert_eq!(result.headers.title, "~");
        assert_eq!(result.body, "");
    }

    #[test]
    fn parse_with_valid_frontmatter() {
        let text = "---\ntitle: dummy_title---\ndummy_body";
        let result = parse::<Headers>(text).unwrap();
        assert_eq!(result.headers.title, "dummy_title");
        assert_eq!(result.body, "dummy_body");
    }
    #[test]
    fn parse_with_valid_frontmatter_crlf() {
        let text = "---\r\ntitle: dummy_title---\r\ndummy_body";
        let result = parse::<Headers>(text).unwrap();
        assert_eq!(result.headers.title, "dummy_title");
        assert_eq!(result.body, "dummy_body");
    }
}
