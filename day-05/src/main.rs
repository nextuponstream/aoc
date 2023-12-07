use helpers::get_input;

#[derive(Debug, Default)]
struct ConversionMap {
    /// src_start, dest_start, range
    mappings: Vec<(u64, u64, u64)>,
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<ConversionMap>,
}

impl Almanac {
    fn new(input: String) -> Self {
        let sections: Vec<&str> = input.split("\n\n").collect();
        // println!("{sections:?}");
        let seeds_input = sections[0];
        let seeds_input = seeds_input.split_once(':').unwrap().1;
        let seeds_numbers: Vec<&str> = seeds_input.split_whitespace().collect();
        let mut seeds: Vec<u64> = vec![];
        for seed in seeds_numbers {
            seeds.push(seed.parse().unwrap())
        }

        let map_inputs = sections.split_at(1).1;
        let mut maps: Vec<ConversionMap> = vec![];
        for input in map_inputs {
            let mut map = ConversionMap::default();
            let mapping_inputs: Vec<&str> = input.split('\n').collect();
            let mapping_inputs = mapping_inputs.split_at(1).1;
            for mapping_input in mapping_inputs {
                if mapping_input == &"" {
                    continue;
                }
                let src_start = helpers::parse_word(mapping_input, 0).parse().unwrap();
                let dest_start = helpers::parse_word(mapping_input, 1).parse().unwrap();
                let range = helpers::parse_word(mapping_input, 2).parse().unwrap();
                let mapping = (src_start, dest_start, range);
                map.mappings.push(mapping);
            }

            maps.push(map)
        }

        Self { seeds, maps }
    }

    fn lowest_location_number(&self) -> u64 {
        let mut lowest = u64::MAX;
        for seed in &self.seeds {
            let mut input: u64 = *seed;

            for map in &self.maps {
                // convert
                if let Some(mapping) = map.mappings.iter().find(|m| {
                    let range = m.1..(m.1 + m.2);
                    range.contains(&input)
                }) {
                    input = mapping.0 + (input - mapping.1);
                }
            }

            // all mapping done
            lowest = lowest.min(input);
        }

        lowest
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

    const TEST_INPUT: &str = "seeds: 79 14 55 13

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
";

    #[test]
    fn almanac_example_1() {
        let almanac = Almanac::new(TEST_INPUT.into());
        assert!(almanac.seeds.contains(&79), "{almanac:?}");
        assert!(almanac.seeds.contains(&14), "{almanac:?}");
        assert!(almanac.seeds.contains(&55), "{almanac:?}");
        assert!(almanac.seeds.contains(&13), "{almanac:?}");
        assert_eq!(almanac.seeds.len(), 4, "{almanac:?}");
        assert_eq!(almanac.lowest_location_number(), 46, "{almanac:?}")
    }
}
