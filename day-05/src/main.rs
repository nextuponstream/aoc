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
        Self::default()
    }

    fn lowest_location_number(&self) -> u32 {
        1000
    }
}

fn main() {
    let input = get_input();
    println!("???");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn almanac_example_1() {
        let almanac = Almanac::new(String::default());
        assert_eq!(almanac.lowest_location_number(), 35, "{almanac:?}")
    }
}
