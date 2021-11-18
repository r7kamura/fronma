use crate::engines::Engine;
use crate::result::Result;

pub struct Json;

impl<T: serde::de::DeserializeOwned> Engine<T> for Json {
    fn parse(value: &str) -> Result<T> {
        serde_json::from_str(value).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::Json;
    use crate::parser::parse_with_engine;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Headers {
        title: String,
    }

    #[test]
    fn parse_with_valid_frontmatter() {
        let text = r#"---
{
    "title": "dummy_title"
}
---
dummy_body
"#;
        let result = parse_with_engine::<Headers, Json>(text).unwrap();
        assert_eq!(result.headers.title, "dummy_title");
        assert_eq!(result.body, "dummy_body\n");
    }
}
