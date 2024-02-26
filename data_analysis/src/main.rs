const ENROLMENTS_PATH: &str = "enrolments.psv";
use std::{collections::{HashMap, HashSet}, io::{stdin, stdout, Write}};
use csv::{ReaderBuilder};
use serde::Deserialize;

fn main() {
    int mut reader = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(ENROLMENTS_PATH)
        .unwrap();

    
}
