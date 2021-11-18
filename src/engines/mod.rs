#[cfg(feature = "default")]
mod yaml;
#[cfg(feature = "default")]
pub use yaml::*;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use json::*;

#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "toml")]
pub use self::toml::*;

use crate::result::Result;

pub trait Engine<T> {
    fn parse(value: &str) -> Result<T>;
}
