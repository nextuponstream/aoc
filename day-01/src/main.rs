// https://adventofcode.com/2023/day/1

/**
--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take
a look. The Elves have even given you a map; on it, they've used stars to mark
the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you
need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day
in the Advent calendar; the second puzzle is unlocked when you complete the
first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough")
and where they're even sending you ("the sky") and why your map looks mostly
blank ("you sure ask a lot of questions") and hang on did you just say the sky
("of course, where do you think snow comes from") when you realize that the
Elves are already loading you into a trebuchet ("please hold still, we need to
strap you in").

As they're making the final adjustments, they discover that their calibration
document (your puzzle input) has been amended by a very young Elf who was
apparently just excited to show off her art skills. Consequently, the Elves are
having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line
originally contained a specific calibration value that the Elves now need to
recover. On each line, the calibration value can be found by combining the first
digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and
77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the
calibration values?
*/
use helpers::{parse_digit, parse_digit_from_text_input, parse_first_of, parse_last_of};

fn parse_calibration_value(input: &str) -> u32 {
    // let first = parse_first_digit(input) * 10;
    // let last = parse_last_digit(input);
    let first = parse_first_of(input, r"(\d|one|two|three|four|five|six|seven|eight|nine)");
    let last = parse_last_of(input, r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)");
    let first = match parse_digit(&first) {
        Some(d) => d * 10,
        None => parse_digit_from_text_input(&first) * 10,
    };
    let last = match parse_digit(&last) {
        Some(d) => d,
        None => {
            let last: String = last.chars().rev().collect();
            parse_digit_from_text_input(&last)
        }
    };

    first + last
}

fn sum_calibration_values(inputs: Vec<&str>) -> u32 {
    let mut sum = 0;
    for input in inputs {
        if input.is_empty() {
            continue;
        }
        sum += parse_calibration_value(input);
    }
    sum
}

fn main() {
    let text_input = String::from_utf8(std::fs::read("input.txt").unwrap()).unwrap();
    let inputs = text_input.split('\n').collect();
    let sum = sum_calibration_values(inputs);
    println!("sum = {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_calibration_values_example() {
        let inputs = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        assert_eq!(sum_calibration_values(inputs), 142)
    }
    #[test]
    fn individual_calibration_values() {
        let inputs = vec![
            ("eightwothree", 83),
            ("two1nine", 29),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        for input in inputs {
            assert_eq!(parse_calibration_value(input.0), input.1)
        }
    }
    #[test]
    fn sum_calibration_values_example2() {
        let inputs = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        assert_eq!(sum_calibration_values(inputs), 281)
    }
}
