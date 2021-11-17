#[derive(Debug)]
pub enum Error {
    MissingBeginningLine,
    MissingEndingLine,
    SerdeYaml(serde_yaml::Error),
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::SerdeYaml(error)
    }
}
