use helpers::get_inputs;

struct EngineSchematic {
    part_numbers: Vec<u32>,
}

impl EngineSchematic {
    fn new(inputs: Vec<String>) -> Self {
        todo!()
    }
}

fn parse_numbers_in_a_line(input: &str) -> Vec<u32> {
    vec![]
}

fn main() {
    let inputs = get_inputs();
    let engine = EngineSchematic::new(inputs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_example() {
        let input = "467..114..";
        let parsed_numbers = parse_numbers_in_a_line(input);
        assert!(parsed_numbers.contains(&467));
        assert!(parsed_numbers.contains(&114));
    }

    #[test]
    fn parse_example_engine_schematic() {
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
    }
}
