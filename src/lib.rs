mod core;
use crate::core::{Definition, get_definition};
use std::io::Error;

pub fn get_word(word: String) -> Result<Definition, Error> {
    get_definition(word.as_str())
}
