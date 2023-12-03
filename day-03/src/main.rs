use helpers::{
    capture_any_words_with_those_characters,
    capture_any_words_with_those_characters_with_positions, check_char_belongs_to_group,
    get_inputs,
};

struct EngineSchematic {
    part_numbers: Vec<u32>,
}

impl EngineSchematic {
    fn new(inputs: Vec<String>) -> Self {
        let mut part_numbers = vec![];
        for i in 0..inputs.len() {
            let line = inputs[i].clone();
            let matches = capture_any_words_with_those_characters_with_positions(&line, r"\d+");

            for (part_number, start, end) in matches {
                let number: u32 = part_number.parse().unwrap();

                if start > 0 {
                    // check left of part number
                    if check_char_belongs_to_group(line.chars().nth(0).unwrap(), r"\d|\.") {
                        part_numbers.push(number);
                        continue;
                    }
                } else if end < line.len() {
                    // check right of part number
                    if check_char_belongs_to_group(
                        line.chars().nth(line.len() - 1).unwrap(),
                        r"\d|\.",
                    ) {
                        part_numbers.push(number);
                        continue;
                    }
                } else {
                    // check above and below
                    todo!()
                }
            }
        }
        Self { part_numbers }
    }
}

fn parse_numbers_in_a_line(input: &str) -> Vec<u32> {
    let raw_numbers = capture_any_words_with_those_characters(input, r"\d+");
    raw_numbers.iter().map(|raw| raw.parse().unwrap()).collect()
}

fn main() {
    let inputs = get_inputs();
    let engine = EngineSchematic::new(inputs);

    println!("sum of parts = {}", engine.part_numbers.iter().sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_example() {
        let input = "467..114..";
        let parsed_numbers = parse_numbers_in_a_line(input);
        assert!(parsed_numbers.contains(&467), "{parsed_numbers:?}");
        assert!(parsed_numbers.contains(&114));
    }

    #[test]
    fn parse_engine_schematic_example_1() {
        let inputs = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
        ];
        let engine = EngineSchematic::new(inputs);

        assert!(engine.part_numbers.contains(&467));
        assert!(engine.part_numbers.contains(&35));
        assert!(engine.part_numbers.contains(&633));
        assert_eq!(engine.part_numbers.len(), 3);
    }

    #[test]
    fn parse_engine_schematic_example_2() {
        let inputs = vec!["467*".to_string()];
        let engine = EngineSchematic::new(inputs);

        assert!(engine.part_numbers.contains(&467));
        assert_eq!(engine.part_numbers.len(), 1);
    }
}
