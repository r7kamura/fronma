use crate::engines::Engine;
use crate::result::Result;

pub struct Yaml;

impl<T: serde::de::DeserializeOwned> Engine<T> for Yaml {
    fn parse(value: &str) -> Result<T> {
        serde_yaml::from_str(value).map_err(Into::into)
    }
}
