#[derive(Default, Debug)]
struct PipeMaze {
    map: Vec<String>,
}

impl PipeMaze {
    fn longest_distance_from_starting_position(&self) -> usize {
        0
    }

    fn new(inputs: Vec<String>) -> Self {
        Self::default()
    }
}

fn main() {
    let inputs = helpers::get_inputs();
    let m = PipeMaze::new(inputs);
    println!(
        "longest distance from starting position = {}",
        m.longest_distance_from_starting_position()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_example1() {
        let inputs = vec![
            ".....".to_string(),
            ".S-7.".to_string(),
            ".|.|.".to_string(),
            ".L-J.".to_string(),
            ".....".to_string(),
        ];

        let m = PipeMaze::new(inputs);
        assert_eq!(m.longest_distance_from_starting_position(), 4);
    }
}
