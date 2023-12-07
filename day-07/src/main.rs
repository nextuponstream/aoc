use helpers::get_inputs;

#[derive(PartialEq, PartialOrd, Debug)]
#[repr(u64)]
enum HandsValue {
    // https://doc.rust-lang.org/stable/std/cmp/trait.PartialOrd.html#derivable
    FiveOf(String) = 7,
    FourOf(String) = 6,
    FullHouse(String) = 5,
    ThreeOf(String) = 4,
    TwoPairs(String) = 3,
    OnePair(String) = 2,
    HighCard(String) = 1,
}

impl std::default::Default for HandsValue {
    fn default() -> Self {
        Self::HighCard(String::default())
    }
}

#[derive(Debug, Default)]
struct Hands {
    /// sorted
    hands: Vec<Hand>,
}

#[derive(Debug, Default)]
struct Hand {
    hand: HandsValue,
    input: String,
    bid: usize,
}

impl Hand {
    fn new(input: String) -> Self {
        let (hand_input, bid_input) = input.split_once(' ').unwrap();
        assert!(hand_input.len() == 5, "bad hand: {input}");
        let mut hand = HandsValue::HighCard(hand_input.to_string());
        let vals = vec![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];

        for v in vals {
            match hand_input.chars().filter(|c| c == &v).count() {
                5 => {
                    hand = HandsValue::FiveOf(hand_input.to_string());
                    break;
                }
                4 => {
                    hand = HandsValue::FourOf(hand_input.to_string());
                    break;
                }
                3 => {
                    let other_chars: Vec<char> = hand_input.chars().filter(|c| c != &v).collect();
                    if other_chars[0] == other_chars[1] {
                        hand = HandsValue::FullHouse(hand_input.to_string())
                    } else {
                        hand = HandsValue::ThreeOf(hand_input.to_string())
                    }
                }
                2 => {
                    let other_chars: Vec<char> = hand_input.chars().filter(|c| c != &v).collect();
                    if other_chars[0] == other_chars[1] && other_chars[1] == other_chars[2] {
                        hand = HandsValue::FullHouse(hand_input.to_string());
                    } else if other_chars[0] == other_chars[1]
                        || other_chars[1] == other_chars[2]
                        || other_chars[2] == other_chars[0]
                    {
                        hand = HandsValue::TwoPairs(hand_input.to_string())
                    } else {
                        hand = HandsValue::OnePair(hand_input.to_string())
                    }
                }
                _ => {}
            }
        }

        Hand {
            hand,
            input: input.clone(),
            bid: bid_input.parse().unwrap(),
        }
    }
}

impl Hands {
    fn new(inputs: Vec<String>) -> Self {
        let mut hands: Vec<Hand> = vec![];
        for input in inputs {
            hands.push(Hand::new(input))
        }

        hands.sort_by(|h1, h2| {
            // println!("compare: {h1:?}, and {h2:?}");
            // println!("idk: {:?}", h1.hand.partial_cmp(&h2.hand).unwrap());

            match (&h1.hand, &h2.hand) {
                (HandsValue::FiveOf(_), HandsValue::FiveOf(_))
                | (HandsValue::FourOf(_), HandsValue::FourOf(_))
                | (HandsValue::FullHouse(_), HandsValue::FullHouse(_))
                | (HandsValue::ThreeOf(_), HandsValue::ThreeOf(_))
                | (HandsValue::TwoPairs(_), HandsValue::TwoPairs(_))
                | (HandsValue::OnePair(_), HandsValue::OnePair(_))
                | (HandsValue::HighCard(_), HandsValue::HighCard(_)) => {
                    // println!("equal???: {h1:?}, and {h2:?}");
                    for i in 0..5 {
                        let vals = [
                            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
                        ];
                        // let c1 = h1.hand
                        // println!("{h1:?}");
                        let c1 = vals
                            .iter()
                            .enumerate()
                            .find(|c| *c.1 == h1.input.chars().nth(i).unwrap())
                            .unwrap();
                        let c2 = vals
                            .iter()
                            .enumerate()
                            .find(|c| *c.1 == h2.input.chars().nth(i).unwrap())
                            .unwrap();

                        if c2.0.cmp(&c1.0) != std::cmp::Ordering::Equal {
                            // println!("h1 vs h2: {:?}", c2.0.cmp(&c1.0));
                            return c2.0.cmp(&c1.0);
                        }
                    }
                    std::cmp::Ordering::Equal
                }
                (_, _) => h1.hand.partial_cmp(&h2.hand).unwrap(),
            }
        });

        // todo, sort again by hand value
        Self { hands }
    }
    fn winnings(&self) -> usize {
        let mut winning: usize = 0;
        for (i, hand) in self.hands.iter().enumerate() {
            let rank = i + 1;
            // let value = rank * hand.bid;
            // println!("{:?}: {} * {} = {value}", hand.hand, rank, hand.bid);
            winning += rank * hand.bid;
        }
        winning
    }
}
fn main() {
    let inputs = get_inputs();
    let hands = Hands::new(inputs);
    println!("winnings = {}", hands.winnings());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let inputs = vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ];
        let hands = Hands::new(inputs);
        assert_eq!(hands.winnings(), 6440, "{hands:?}");
    }

    #[test]
    fn handTypeSort() {
        assert!(HandsValue::FiveOf(String::default()) > HandsValue::FourOf(String::default()));
    }

    #[test]
    fn full_house() {
        let hand = Hand::new("QQQJJ 2".to_string());
        if let HandsValue::FullHouse(_) = hand.hand {
        } else {
            panic!("oh no QQQJJ not a full house, {hand:?}")
        }
        let hand = Hand::new("QJQJQ 2".to_string());
        if let HandsValue::FullHouse(_) = hand.hand {
        } else {
            panic!("oh no QQQJJ not a full house, {hand:?}")
        }
        let hand = Hand::new("JJQQQ 2".to_string());
        if let HandsValue::FullHouse(_) = hand.hand {
        } else {
            panic!("oh no QQQJJ not a full house, {hand:?}")
        }
    }
    #[test]
    fn three_of() {
        let hand = Hand::new("QQQAJ 2".to_string());
        if let HandsValue::ThreeOf(_) = hand.hand {
        } else {
            panic!("oh no QQQAJ is not a triple {hand:?}")
        }
    }
    #[test]
    fn pair() {
        let hand = Hand::new("QQ9AJ 2".to_string());
        if let HandsValue::OnePair(_) = hand.hand {
        } else {
            panic!("oh no QQ9AJ is not a pair {hand:?}")
        }
    }
}
