//! Some complementory functions for week 3 exercise caesar
//! 
//! Provides ways to shift
//! There are sevearl advantages:
//! Easy to use
//! Code is reusable

/// default shift is 5
const DEFAULT_SHIFT: i32 = 5;
/// uppercase a is 65
const UPPERCASE_A: i32 = 65;
/// lowercase a is 97
const LOWERCASE_A: i32 = 97;
/// alphabet size is 26
const ALPHABET_SIZE: i32 = 26;

/// A caesar shift function that generates a list of encrypted lines
pub fn caesar_shift(shift_by: Option<i32>, lines: Vec<String>) -> Vec<String> {
    let shift_number = shift_by.unwrap_or(DEFAULT_SHIFT);
    
    // no idea what this is doing? Ask the forums and/or 
    // look back at the functional programming lectures!
    lines
        .iter()
        .map(|line| shift(shift_number, line.to_string()))
        .collect()
}

fn shift(shift_by: i32, line: String) -> String {
    let mut result: Vec<char> = Vec::new();

    // turn shift_by into a positive number between 0 and 25
    let shift_by = shift_by % ALPHABET_SIZE + ALPHABET_SIZE;

    line.chars().for_each(|c| {
        let ascii = c as i32;

        if ('A'..='Z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - UPPERCASE_A) + shift_by, ALPHABET_SIZE) + UPPERCASE_A,
            ));
        } else if ('a'..='z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - LOWERCASE_A) + shift_by, ALPHABET_SIZE) + LOWERCASE_A,
            ));
        } else {
            result.push(c)
        }
    });

    result.iter().collect()
}

fn abs_modulo(a: i32, b: i32) -> i32 {
    (a % b).abs()
}

fn to_ascii(i: i32) -> char {
    char::from_u32(i as u32).unwrap()
}
