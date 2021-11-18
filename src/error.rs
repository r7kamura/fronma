#[derive(Debug)]
pub enum Error {
    MissingBeginningLine,
    MissingEndingLine,
    #[cfg(feature = "default")]
    SerdeYaml(serde_yaml::Error),
    #[cfg(feature = "json")]
    SerdeJson(serde_json::Error),
    #[cfg(feature = "toml")]
    Toml(toml::de::Error),
}

#[cfg(feature = "default")]
impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::SerdeYaml(error)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJson(error)
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::Toml(error)
    }
}
