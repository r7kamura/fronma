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
    let line_pattern = "---\n";
    if !text.starts_with(line_pattern) {
        return Err(Error::MissingBeginningLine);
    }

    let slice = &text[line_pattern.len()..];
    let index_of_ending_line = slice.find(line_pattern).ok_or(Error::MissingEndingLine)?;
    let headers = E::parse(&slice[..index_of_ending_line])?;
    let body = &slice[(index_of_ending_line + line_pattern.len())..];
    Ok(ParsedData { body, headers })
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
    fn parse_with_empty_frontmatter() {
        let text = "---\n---\n";
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
    fn parse_with_unknown_field() {
        let text = "---\ndate: 2000-01-01\ntitle: dummy_title\n---\n";
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
    fn parse_with_valid_frontmatter() {
        let text = "---\ntitle: dummy_title---\ndummy_body";
        let result = parse::<Headers>(text).unwrap();
        assert_eq!(result.headers.title, "dummy_title");
        assert_eq!(result.body, "dummy_body");
    }
}
