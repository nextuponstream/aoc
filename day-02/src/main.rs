use helpers::{get_inputs, parse_word};

#[derive(Default, Debug)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

impl Game {
    fn max_red(&self) -> u32 {
        self.pulls
            .iter()
            .fold(0, |max_val, val| max_val.max(val.red))
    }
    fn max_green(&self) -> u32 {
        self.pulls
            .iter()
            .fold(0, |max_val, val| max_val.max(val.green))
    }
    fn max_blue(&self) -> u32 {
        self.pulls
            .iter()
            .fold(0, |max_val, val| max_val.max(val.blue))
    }
    fn possible_game(&self) -> bool {
        self.max_red() <= 12 && self.max_green() <= 13 && self.max_blue() <= 14
    }
    fn power(&self) -> u32 {
        // minimum number of cubes for each types multiplied together
        self.max_red() * self.max_green() * self.max_blue()
    }
}

fn parse_pull_set(input: &str) -> Pull {
    let pull_sets: Vec<&str> = input.split(',').collect();
    let pulls: Vec<(u32, Colors)> = pull_sets.iter().map(|pull| parse_many_of(pull)).collect();
    let mut pull = Pull::default();
    for p in pulls {
        match p.1 {
            Colors::Red => pull.red += p.0,
            Colors::Green => pull.green += p.0,
            Colors::Blue => pull.blue += p.0,
        }
    }
    pull
}
enum Colors {
    Red,
    Green,
    Blue,
}
fn parse_many_of(input: &str) -> (u32, Colors) {
    let quantity = parse_word(input, 0);
    let color = parse_word(input, 1);
    let color = match color {
        "red" => Colors::Red,
        "green" => Colors::Green,
        "blue" => Colors::Blue,
        _ => unreachable!(),
    };

    (quantity.parse().unwrap(), color)
}

fn parse_game(input: &str) -> Game {
    let (game_info_input, pulls_input) = input.split_once(':').unwrap();

    let id = parse_word(game_info_input, 1);
    let id: u32 = id.parse().unwrap();

    let pulls_input: Vec<&str> = pulls_input.split(';').collect();
    let pulls_input: Vec<Vec<&str>> = pulls_input
        .iter()
        .map(|p_input| p_input.split(',').collect())
        .collect();

    let mut pulls = vec![];
    for input in pulls_input {
        for color_input in input {
            let pull = parse_pull_set(color_input);
            pulls.push(pull);
        }
    }

    Game { id, pulls }
}

fn main() {
    let inputs = get_inputs();
    let sum_of_valid_game_ids: u32 = inputs
        .iter()
        .map(|game_input| parse_game(game_input))
        .filter(|game| game.possible_game())
        .map(|game| game.id)
        .sum::<u32>();
    println!("sum of valid game ids = {sum_of_valid_game_ids}");

    // part 2
    let sum_of_powers: u32 = inputs
        .iter()
        .map(|game_input| parse_game(game_input))
        .map(|game| game.power())
        .sum::<u32>();
    println!("sum of power is = {sum_of_powers}");
}

#[cfg(test)]
mod tests {
    use super::*;

    // irrelevant for part 1
    // #[test]
    // fn parse_game_example() {
    //     let game = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    //     assert_eq!(game.id, 1);
    //     assert_eq!(game.pulls[0].red, 4, "{game:?}");
    //     assert_eq!(game.pulls[0].green, 0);
    //     assert_eq!(game.pulls[0].blue, 3);
    //     assert_eq!(game.pulls[1].red, 1);
    //     assert_eq!(game.pulls[1].green, 2);
    //     assert_eq!(game.pulls[1].blue, 6);
    //     assert_eq!(game.pulls[2].red, 0);
    //     assert_eq!(game.pulls[2].green, 2);
    //     assert_eq!(game.pulls[2].blue, 0);
    // }

    #[test]
    fn example_games() {
        let games: Vec<Game> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .into_iter()
        .map(parse_game)
        .collect();
        let winning_games: Vec<Game> = games.into_iter().filter(|g| g.possible_game()).collect();
        let ids: Vec<u32> = winning_games.iter().map(|g| g.id).collect();
        let sum_ids: u32 = ids.iter().sum();
        assert_eq!(sum_ids, 8)
    }

    #[test]
    fn parse_pull_set_example() {
        let pull = parse_pull_set("3 blue, 4 red");
        assert_eq!(pull.red, 4);
        assert_eq!(pull.blue, 3);
        assert_eq!(pull.green, 0);
    }

    #[test]
    fn power_examples() {
        let game_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(game_input);

        assert_eq!(game.power(), 48);
    }

    #[test]
    fn sum_powers_example() {
        let games: Vec<Game> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .into_iter()
        .map(parse_game)
        .collect();
        let sum_of_powers: u32 = games.iter().map(|game| game.power()).sum::<u32>();
        assert_eq!(sum_of_powers, 2286)
    }
}
