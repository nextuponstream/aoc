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
/// # Panics
/// Asserts that there is smth to find (or crashes)
#[must_use]
pub fn parse_first_of(input: &str, regex: &str) -> String {
    let re = Regex::new(regex).unwrap();
    let parsed_digit = re.find(input).unwrap().as_str();
    parsed_digit.to_string()
}

/// # Panics
/// Asserts that there is a digit (or crashes)
#[must_use]
pub fn parse_last_digit(input: &str) -> u32 {
    let reversed_input: String = input.chars().rev().collect();
    parse_first_digit(&reversed_input)
}
/// # Panics
/// Asserts that there is smth to find (or crashes)
#[must_use]
pub fn parse_last_of(input: &str, regex: &str) -> String {
    let reversed_input: String = input.chars().rev().collect();
    parse_first_of(&reversed_input, regex)
}

/// # Panics
/// Asserts that there is a digit (or crashes)
#[must_use]
pub fn parse_digit(input: &str) -> Option<u32> {
    let re = Regex::new(r"\d").unwrap();
    re.find(input)
        .map(|parsed_digit| parsed_digit.as_str().parse::<u32>().unwrap())
}
/// # Panics
/// Asserts that there is a digit (or crashes)
#[must_use]
pub fn parse_digit_from_text_input(input: &str) -> u32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }
}

#[must_use]
/// Example: given regex \d+, in a line like "467..114.." returns "467" & "114"
/// # Panics
/// when no match
pub fn capture_any_words_with_those_characters(input: &str, regex: &str) -> Vec<String> {
    let re = Regex::new(regex).unwrap();
    let matches: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    matches
        .iter()
        .map(std::string::ToString::to_string)
        .collect()
}
#[must_use]
/// Example: given regex \d*, in a line like "467..114.." returns "467" & "114"
/// # Panics
/// when no match
pub fn capture_any_words_with_those_characters_with_positions(
    input: &str,
    regex: &str,
) -> Vec<(String, usize, usize)> {
    let re = Regex::new(regex).unwrap();
    let matches: Vec<(String, usize, usize)> = re
        .find_iter(input)
        .map(|m| (m.as_str().to_string(), m.start(), m.end()))
        .collect();
    matches
}

/// Parse first, second... word given `position`
/// # Panics
/// when no match
#[must_use]
pub fn parse_word(input: &str, pos: usize) -> &str {
    let re = Regex::new(r"\w+").unwrap();
    let matches: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    matches[pos]
}

/// Returns list of inputs from 'input.txt'
/// # Panics
/// when there is no file to read
#[must_use]
pub fn get_inputs() -> Vec<String> {
    let text_input = String::from_utf8(std::fs::read("input.txt").unwrap()).unwrap();
    let inputs: Vec<&str> = text_input.split('\n').collect();
    let inputs: Vec<String> = inputs
        .iter()
        .map(std::string::ToString::to_string)
        .filter(|s| !s.is_empty())
        .collect();
    inputs
}

/// Check if character belongs to set
/// # Panics
/// when regex is faulty
#[must_use]
pub fn check_char_belongs_to_group(char: char, regex: &str) -> bool {
    let re = Regex::new(regex).unwrap();
    matches!(re.find(char.to_string().as_str()), Some(_m))
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

    #[test]
    fn get_last_last() {
        let input = "abc2";
        let result = parse_last_digit(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn get_last_not_last() {
        let input = "ab2c";
        let result = parse_last_digit(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn get_last_edge_case() {
        let input = "2abc";
        let result = parse_last_digit(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn get_last_with_many() {
        let input = "ab12c";
        let result = parse_last_digit(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn parse_first_of_example() {
        let input = "two1nine";
        let first = parse_first_of(input, r"(\d|one|two|three|four|five|six|seven|eight|nine)");
        assert_eq!(first, "two");
    }
    #[test]
    fn parse_last_of_example() {
        let input = "two1nine";
        let last = parse_last_of(input, r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)");
        assert_eq!(last, "enin");
    }
    #[test]
    fn parse_second_word() {
        let output = parse_word("Game 1", 1);
        assert_eq!(output, "1");
    }
    #[test]
    fn parse_words_given_regex_example() {
        let input = "467..114..";
        let matches = capture_any_words_with_those_characters(input, r"\d+");
        assert!(matches.contains(&"467".to_string()));
        assert!(matches.contains(&"114".to_string()));
    }
    #[test]
    fn parse_words_given_regex_with_position_example() {
        let input = "467..114..";
        let matches = capture_any_words_with_those_characters_with_positions(input, r"\d+");
        assert_eq!(matches[0].0, "467");
        assert_eq!(matches[0].1, 0);
        assert_eq!(matches[0].2, 3);
        assert_eq!(matches[1].0, "114");
        assert_eq!(matches[1].1, 5);
        assert_eq!(matches[1].2, 8);
    }
    #[test]
    fn belongs_to_whitelist_example() {
        assert!(check_char_belongs_to_group('.', r"\d|\."));
        assert!(check_char_belongs_to_group('3', r"\d|\."));
        assert!(!check_char_belongs_to_group('#', r"\d|\."));
    }
}
