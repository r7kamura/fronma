mod yaml;

pub use yaml::*;

use crate::result::Result;

pub trait Engine<T> {
    fn parse(value: &str) -> Result<T>;
}
