pub fn hello_world() -> String {
    String::from("Hello world from helper")
}

use regex::Regex;

// Asserts that there is a digit (or crashes)
pub fn parse_first_digit(input: &str) -> u32 {
    let re = Regex::new(r"\d").unwrap();
    let parsed_digit = re.find(input).unwrap().as_str();
    parsed_digit.parse::<u32>().unwrap()
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
