use helpers::parse_word;

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
    let (game_info, pulls) = input.split_once(':').unwrap();
    let id = parse_word(game_info, 1);
    let id: u32 = id.parse().unwrap();
    Game { id, pulls: vec![] }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_example() {
        let game = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game.id, 1);
        assert_eq!(game.pulls[0].red, 4);
        assert_eq!(game.pulls[0].green, 0);
        assert_eq!(game.pulls[0].blue, 3);
        assert_eq!(game.pulls[1].red, 1);
        assert_eq!(game.pulls[1].green, 2);
        assert_eq!(game.pulls[1].blue, 6);
        assert_eq!(game.pulls[2].red, 0);
        assert_eq!(game.pulls[2].green, 2);
        assert_eq!(game.pulls[2].blue, 0);
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
