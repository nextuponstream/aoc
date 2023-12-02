//! helpers file for each aoc challenge

#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]

use regex::Regex;

/// # Panics
/// Asserts that there is a digit (or crashes)
#[must_use]
pub fn parse_first_digit(input: &str) -> u32 {
    let re = Regex::new(r"\d").unwrap();
    let parsed_digit = re.find(input).unwrap().as_str();
    parsed_digit.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_first_first() {
        let input = "1abc";
        let result = parse_first_digit(input);
        assert_eq!(result, 1);
    }
    #[test]
    fn get_first_not_first() {
        let input = "ab1c";
        let result = parse_first_digit(input);
        assert_eq!(result, 1);
    }
    #[test]
    fn get_first_edge_case() {
        let input = "abc1";
        let result = parse_first_digit(input);
        assert_eq!(result, 1);
    }
    #[test]
    fn get_first_with_many() {
        let input = "ab12c";
        let result = parse_first_digit(input);
        assert_eq!(result, 1);
    }
}
