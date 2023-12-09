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
    fn steps(&self) -> u64 {
        // second number is number of steps to reach Z for first time
        let mut nodes: Vec<(String, u64)> = self
            .mappings
            .iter()
            .map(|m| (m.0.to_string(), 0))
            .filter(|n| n.0.ends_with('A'))
            .collect();
        let initial_nodes_count = nodes.len();
        println!("initial starting nodes: {initial_nodes_count}");
        let mut instruction_counter = 0;
        let mut step_count = 0;

        while nodes.iter().filter(|n| n.0.ends_with('Z')).count() != initial_nodes_count
            && nodes.iter().filter(|n| n.1 > 0).count() != initial_nodes_count
        {
            // println!("{self:?}");
            nodes = nodes
                .iter()
                .map(|n| {
                    let mapping = self.mappings.iter().find(|m| m.0 == n.0).unwrap();
                    let m = match self.instructions[instruction_counter] {
                        false => mapping.1 .0.to_string(),
                        true => mapping.1 .1.to_string(),
                    };
                    if n.1 == 0 && m.ends_with('Z') {
                        (m, step_count + 1)
                    } else {
                        (m, n.1)
                    }
                })
                .collect();
            instruction_counter = (instruction_counter + 1) % self.instructions.len();
            step_count += 1;
        }

        if nodes.iter().filter(|n| n.1 > 0).count() == initial_nodes_count {
            // you can observe in the example that starting node 1 reaches Z
            // every 2 steps and starting node 2 in 3. Then the answer can be
            // computed rather than brute forced
            nodes.iter().map(|n| n.1).fold(1, num::integer::lcm)
        } else {
            step_count
        }
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
