#[derive(Debug, Default)]
struct NetworkMap {
    instructions: Vec<bool>,
    mappings: Vec<(String, (String, String))>,
}

impl NetworkMap {
    fn new(input: String) -> Self {
        let (instruction_input, mappings_input) = input.split_once("\n\n").unwrap();

        let mut instructions = vec![];
        for i in instruction_input.chars() {
            instructions.push(match i {
                'L' => false,
                'R' => true,
                _ => unreachable!(),
            })
        }
        let mut mappings = vec![];
        for m in mappings_input.split('\n') {
            if m.is_empty() {
                continue;
            }
            let splits: Vec<String> = helpers::capture_any_words_with_those_characters(m, r"\w+");
            // println!("{m}");
            let key = splits[0].to_string();
            let left = splits[1].to_string();
            let right = splits[2].to_string();

            mappings.push((key, (left, right)));
        }
        Self {
            instructions,
            mappings,
        }
    }
    fn steps(&self) -> usize {
        let mut step = "AAA".to_string();
        let mut instruction_counter = 0;
        let mut step_count = 0;

        while step != "ZZZ" {
            // println!("{self:?}");
            let mapping = self.mappings.iter().find(|m| m.0 == step).unwrap();
            step = match self.instructions[instruction_counter] {
                false => mapping.1 .0.to_string(),
                true => mapping.1 .1.to_string(),
            };
            instruction_counter = (instruction_counter + 1) % self.instructions.len();
            step_count += 1;
        }

        step_count
    }
}

fn main() {
    let input = helpers::get_input();
    let m = NetworkMap::new(input);
    println!("steps to ZZZ: {}", m.steps());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let m = NetworkMap::new(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
                .to_string(),
        );

        assert_eq!(m.instructions[0], true);
        assert_eq!(m.instructions[1], false);
        assert_eq!(m.instructions.len(), 2);
        assert_eq!(m.mappings.len(), 7);
        assert_eq!(m.steps(), 2);
    }
    #[test]
    fn example_2() {
        let m = NetworkMap::new(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
                .to_string(),
        );

        assert_eq!(m.instructions[0], false);
        assert_eq!(m.instructions[1], false);
        assert_eq!(m.instructions[2], true);
        assert_eq!(m.instructions.len(), 3);
        assert_eq!(m.mappings.len(), 3);
        assert_eq!(m.steps(), 6);
    }
}
