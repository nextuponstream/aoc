use helpers::get_inputs;

#[derive(Debug, Default)]
struct RaceRecords {
    times: Vec<u32>,
    distances: Vec<u32>,
}

impl RaceRecords {
    fn new(inputs: Vec<String>) -> Self {
        let times: Vec<u32> = inputs[0]
            .clone()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|ti| ti.parse::<u32>().unwrap())
            .collect();
        let distances: Vec<u32> = inputs[1]
            .clone()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|ti| ti.parse::<u32>().unwrap())
            .collect();

        Self { times, distances }
    }
    fn magic_number(&self) -> u32 {
        let mut magic = vec![];
        for race in 0..self.times.len() {
            let mut winning_strat_counter = 0;
            for hold_duration in 0..self.times[race] {
                if hold_duration * (self.times[race] - hold_duration) > self.distances[race] {
                    winning_strat_counter += 1;
                }
            }
            magic.push(winning_strat_counter);
        }

        magic.iter().product::<u32>()
    }
}

fn main() {
    let inputs = get_inputs();
    let race_record = RaceRecords::new(inputs);
    println!("product of strats = {}", race_record.magic_number());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let inputs = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];
        let race_record = RaceRecords::new(inputs);
        assert_eq!(race_record.times[0], 7, "{race_record:?}");
        assert_eq!(race_record.distances[0], 9, "{race_record:?}");
        assert_eq!(race_record.magic_number(), 288, "{race_record:?}")
    }
}
