#[derive(Debug, Default)]
struct OasisReport {
    reports: Vec<Vec<i64>>,
}

impl OasisReport {
    fn new(inputs: Vec<String>) -> Self {
        let mut reports = vec![];
        for input in inputs {
            let numbers: Vec<i64> = input
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect();
            reports.push(numbers);
        }
        Self { reports }
    }

    fn sum_of_extrapolated_values(&self) -> i64 {
        self.reports
            .iter()
            .map(|report| {
                // println!("...");
                let mut extrapolations_over_time: Vec<Vec<i64>> = vec![report.clone()];
                let mut extrapolation: Vec<i64> = vec![];
                for i in 0..report.len() - 1 {
                    let diff = report[i + 1] - report[i];
                    extrapolation.push(diff)
                }
                extrapolations_over_time.push(extrapolation);
                while extrapolations_over_time
                    .last()
                    .unwrap()
                    .iter()
                    .any(|v| *v != 0)
                {
                    let last_extrapolation = extrapolations_over_time.last().unwrap().clone();
                    let mut new_extrapolation = vec![];
                    // println!("before {last_extrapolation:?}");
                    for i in 0..last_extrapolation.len() - 1 {
                        let diff = last_extrapolation[i + 1] - last_extrapolation[i];
                        new_extrapolation.push(diff)
                    }
                    // println!("after {new_extrapolation:?}");
                    extrapolations_over_time.push(new_extrapolation);
                }

                // println!("ok? {:?}", extrapolations_over_time.last().unwrap());

                extrapolations_over_time.last_mut().unwrap().push(0);

                for i in (1..extrapolations_over_time.len()).rev() {
                    let to_add = extrapolations_over_time[i].last().unwrap();
                    let next_val = extrapolations_over_time[i - 1].last().unwrap() + to_add;
                    extrapolations_over_time[i - 1].push(next_val);
                }

                // println!("{extrapolations_over_time:?}");
                *extrapolations_over_time[0].last().unwrap()
            })
            .sum()
    }
}

fn main() {
    let inputs = helpers::get_inputs();
    let report = OasisReport::new(inputs);
    println!("sum is {}", report.sum_of_extrapolated_values());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        let inputs = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .split('\n')
            .map(|n| n.to_string())
            .collect();
        let report = OasisReport::new(inputs);
        assert_eq!(report.sum_of_extrapolated_values(), 114);
    }
}
