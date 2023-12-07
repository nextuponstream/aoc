use helpers::get_input;

#[derive(Debug)]
struct ConversionMap {
    /// src_start, dest_start, range
    mappings: Vec<(u32, u32, u32)>,
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<ConversionMap>,
}

impl Almanac {
    fn new(input: String) -> Self {
        let sections: Vec<&str> = input.split("\n\n").collect();
        println!("{sections:?}");
        let seeds_input = sections[0];
        let seeds_input = seeds_input.split_once(':').unwrap().1;
        let seeds_numbers: Vec<&str> = seeds_input.split_whitespace().collect();
        let mut seeds: Vec<u32> = vec![];
        for seed in seeds_numbers {
            seeds.push(seed.parse().unwrap())
        }
        Self {
            seeds,
            maps: vec![],
        }
    }

    fn lowest_location_number(&self) -> u32 {
        1000
    }
}

fn main() {
    let input = get_input();
    let almanac = Almanac::new(input);
    println!(
        "lowest location number: {}",
        almanac.lowest_location_number()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn almanac_example_1() {
        let almanac = Almanac::new(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
            .into(),
        );
        assert!(almanac.seeds.contains(&79), "{almanac:?}");
        assert!(almanac.seeds.contains(&14), "{almanac:?}");
        assert!(almanac.seeds.contains(&55), "{almanac:?}");
        assert!(almanac.seeds.contains(&13), "{almanac:?}");
        assert_eq!(almanac.seeds.len(), 4, "{almanac:?}");
        assert_eq!(almanac.lowest_location_number(), 35, "{almanac:?}")
    }
}
