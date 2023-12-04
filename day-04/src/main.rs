use helpers::{capture_any_words_with_those_characters, get_inputs};

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn point_worth(&self) -> u32 {
        0
    }
}

#[derive(Debug, Default)]
struct PileOfCards {
    cards: Vec<Card>,
}

impl PileOfCards {
    fn point_worth(&self) -> u32 {
        self.cards
            .iter()
            .map(|c| c.point_worth())
            .collect::<Vec<u32>>()
            .iter()
            .sum()
    }

    fn new(card_inputs: Vec<String>) -> Self {
        let mut cards = vec![];
        for input in card_inputs {
            let number_inputs = input.split_once(':').unwrap().1;
            let (winning_numbers_input, numbers_input) = number_inputs.split_once('|').unwrap();

            let winning_numbers =
                capture_any_words_with_those_characters(winning_numbers_input, r"\d+");
            let winning_numbers: Vec<u32> =
                winning_numbers.iter().map(|n| n.parse().unwrap()).collect();

            let numbers = capture_any_words_with_those_characters(numbers_input, r"\d+");
            let numbers: Vec<u32> = numbers.iter().map(|n| n.parse().unwrap()).collect();

            let card = Card {
                winning_numbers,
                numbers,
            };
            cards.push(card);
        }
        Self { cards }
    }
}

fn main() {
    let inputs = get_inputs();
    let pile = PileOfCards::new(inputs);
    println!("points worth: {}", pile.point_worth());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pile_worth_example() {
        let inputs = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
        let pile = PileOfCards::new(inputs);

        assert_eq!(pile.point_worth(), 13)
    }
}
