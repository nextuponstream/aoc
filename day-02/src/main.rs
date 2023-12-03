struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

impl Game {
    fn possible_game(&self) -> bool {
        false
    }
}

fn parse_game(input: &str) -> Game {
    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_example() {
        todo!()
    }

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
}
