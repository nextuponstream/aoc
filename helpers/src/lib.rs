pub fn hello_world() -> String {
    String::from("Hello world from helper")
}

use regex::Regex;

pub fn parse_first_digit(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn get_easy() {
        let input = "1abc";
        let result = parse_first_digit(input);
        assert_eq!(result, 1)
    }
}
