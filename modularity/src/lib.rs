use std::env;
use std::num::ParseIntError;

pub const DEFAULT_NUM: u8 = 10;
pub const ERROR_MESSAGE: &str = "Please enter a valid size";

use crate::constants;

pub fn first_argument() -> Result<usize, ParseIntError> {
    let args: Vec<String> = env::args().collect();
    
    match args.get(1) {
        Some(s) => s.parse::<usize>(),
        None => Ok(constants::DEFAULT_NUM.into()),
    }
}