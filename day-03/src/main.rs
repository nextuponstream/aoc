use helpers::{
    capture_any_words_with_those_characters_with_positions, check_char_belongs_to_group,
    get_inputs, parse_first_of, parse_last_of,
};

#[derive(Debug, Default)]
struct Gear {
    neighbor_part_numbers: Vec<u32>,
}

#[derive(Debug)]
struct EngineSchematic {
    part_numbers: Vec<u32>,
    gears: Vec<Gear>,
}

impl EngineSchematic {
    fn new(inputs: Vec<String>) -> Self {
        let mut part_numbers = vec![];
        let mut gears: Vec<Gear> = vec![];
        for i in 0..inputs.len() {
            let line = inputs[i].clone();
            let part_numbers_in_line =
                capture_any_words_with_those_characters_with_positions(&line, r"\d+");

            for (part_number, start, end) in part_numbers_in_line {
                // println!("{}", part_number);
                let number: u32 = part_number.parse().unwrap();
                let start_index = if start == 0 { 0 } else { start - 1 };
                let end_index = if end == line.len() {
                    line.len()
                } else {
                    end + 1
                };
                // println!("---");
                // println!("start: {start_index}");
                // println!("end  : {end_index}");

                if start > 0 {
                    // check left of part number
                    if !check_char_belongs_to_group(
                        line.chars().nth(0.max(start - 1)).unwrap(),
                        r"\d|\.",
                    ) {
                        part_numbers.push(number);
                        continue;
                    }
                }
                if end < line.len() {
                    // check right of part number
                    // println!("right {}", line.chars().nth(line.len() - 1).unwrap());
                    if !check_char_belongs_to_group(line.chars().nth(end).unwrap(), r"\d|\.") {
                        part_numbers.push(number);
                        continue;
                    }
                }

                if i > 0 {
                    let upper_line = inputs[i - 1].clone();
                    // println!("upper  {upper_line}");
                    // println!("line ^ {line}");
                    let mut found = false;
                    for idx in start_index..end_index {
                        let c = upper_line.chars().nth(idx).unwrap();
                        if !check_char_belongs_to_group(c, r"\d|\.") {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        // println!("found in upper for {number}");
                        part_numbers.push(number);
                        continue;
                    }
                }
                if i < inputs.len() - 1 {
                    let bottom_line = inputs[i + 1].clone();
                    // println!("line v {line}");
                    // println!("bottom {bottom_line}");
                    let mut found = false;
                    for idx in start_index..end_index {
                        let c = bottom_line.chars().nth(idx).unwrap();
                        if !check_char_belongs_to_group(c, r"\d|\.") {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        // println!("found in bottom for {number}");
                        part_numbers.push(number);
                        continue;
                    }
                }
            }

            for (j, maybe_gear) in line.chars().enumerate() {
                let mut gear = Gear::default();
                if maybe_gear == '*' {
                    // left
                    if j > 0 && line.chars().nth(j - 1).unwrap().is_ascii_digit() {
                        let left_part = line.split_at(j).0;
                        let reversed_parse_number_input = parse_last_of(left_part, r"\d+");
                        let part_number_input: String =
                            reversed_parse_number_input.chars().rev().collect();
                        let part_number: u32 = part_number_input.parse().unwrap();
                        gear.neighbor_part_numbers.push(part_number)
                    }
                    // right
                    if j < line.len() - 1 && line.chars().nth(j + 1).unwrap().is_ascii_digit() {
                        let right_part = line.split_at(j).1;
                        // println!("{right_part}");
                        let part_number_input = parse_first_of(right_part, r"\d+");
                        let part_number: u32 = part_number_input.parse().unwrap();
                        gear.neighbor_part_numbers.push(part_number)
                    }
                    // top
                    if i > 0 {
                        let upper_line = inputs[i - 1].clone();
                        let raw_number_inputs =
                            capture_any_words_with_those_characters_with_positions(
                                &upper_line,
                                r"\d+",
                            );
                        for (n, start, end) in raw_number_inputs {
                            // println!("start {start}");
                            // println!("end {end}");
                            // println!("j {j}");
                            if end >= j && end <= (j + 1) + (end - start) {
                                gear.neighbor_part_numbers.push(n.parse().unwrap());
                            }
                        }
                    }
                    // bot
                    if i < inputs.len() - 1 {
                        let bottom_line = inputs[i + 1].clone();
                        let raw_number_inputs =
                            capture_any_words_with_those_characters_with_positions(
                                &bottom_line,
                                r"\d+",
                            );
                        for (n, start, end) in raw_number_inputs {
                            if end >= j && end <= (j + 1) + (end - start) {
                                gear.neighbor_part_numbers.push(n.parse().unwrap());
                            }
                        }
                    }

                    // println!("{gear:?}");
                    if gear.neighbor_part_numbers.len() == 2 {
                        gears.push(gear);
                    }
                }
            }
        }
        Self {
            part_numbers,
            gears,
        }
    }

    fn sum_of_part_number(&self) -> u32 {
        self.part_numbers.iter().sum::<u32>()
    }

    fn sum_of_gear_ratios(&self) -> u32 {
        self.gears
            .iter()
            .map(|g| g.neighbor_part_numbers[0] * g.neighbor_part_numbers[1])
            .sum::<u32>()
    }
}

fn main() {
    let inputs = get_inputs();
    let engine = EngineSchematic::new(inputs);

    println!("sum of parts = {}", engine.sum_of_part_number());
    println!("sum of gear ratios = {}", engine.sum_of_gear_ratios())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(engine.sum_of_part_number(), 1135);
    }
    #[test]
    fn parse_engine_schematic_example_7() {
        let inputs = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        let engine = EngineSchematic::new(inputs);

        assert_eq!(engine.part_numbers.len(), 8, "{engine:?}");
        assert_eq!(engine.sum_of_part_number(), 4361, "{engine:?}");
        assert_eq!(engine.sum_of_gear_ratios(), 467835, "{engine:?}");
    }
    #[test]
    fn parse_engine_schematic_example_2() {
        let inputs = vec!["467*".to_string()];
        let engine = EngineSchematic::new(inputs);

        assert!(engine.part_numbers.contains(&467), "{engine:?}");
        assert_eq!(engine.part_numbers.len(), 1);
    }
    #[test]
    fn parse_engine_schematic_example_3() {
        let inputs = vec!["*467..".to_string()];
        let engine = EngineSchematic::new(inputs);

        assert!(engine.part_numbers.contains(&467), "{engine:?}");
        assert_eq!(engine.part_numbers.len(), 1);
    }
    #[test]
    fn parse_engine_schematic_example_4() {
        let inputs = vec!["..*...".to_string(), ".467..".to_string()];
        let engine = EngineSchematic::new(inputs);

        assert!(engine.part_numbers.contains(&467), "{engine:?}");
        assert_eq!(engine.part_numbers.len(), 1);
    }
    #[test]
    fn parse_engine_schematic_example_5() {
        let inputs = vec![".467..".to_string(), "..*...".to_string()];
        let engine = EngineSchematic::new(inputs);

        assert!(engine.part_numbers.contains(&467), "{engine:?}");
        assert_eq!(engine.part_numbers.len(), 1);
    }
    #[test]
    fn parse_engine_schematic_gear_example_1() {
        let inputs = vec!["....3*2...".to_string()];
        let engine = EngineSchematic::new(inputs);

        assert_eq!(engine.sum_of_gear_ratios(), 6, "{engine:?}");
    }
    #[test]
    fn parse_engine_schematic_gear_example_2() {
        let inputs = vec!["...10*3...".to_string()];
        let engine = EngineSchematic::new(inputs);

        assert_eq!(engine.sum_of_gear_ratios(), 30, "{engine:?}");
    }
    #[test]
    fn parse_engine_schematic_gear_example_3() {
        let inputs = vec!["617*......".to_string(), ".....+.58.".to_string()];
        let engine = EngineSchematic::new(inputs);

        assert_eq!(engine.sum_of_gear_ratios(), 0, "{engine:?}");
    }
    #[test]
    fn parse_engine_schematic_gear_example_4() {
        let inputs = vec![
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        let engine = EngineSchematic::new(inputs);

        assert_eq!(engine.sum_of_gear_ratios(), 755 * 598, "{engine:?}");
    }
}
