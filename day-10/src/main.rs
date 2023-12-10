use std::str::FromStr;

#[derive(Default, Debug)]
struct PipeMaze {
    map: Vec<String>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
    Arrived,
}

enum Pipes {
    Vertical,
    Horizontal,
    WestToNorth,
    WestToSouth,
    EastToNorth,
    EastToSouth,
    NotAPipe,
    Start,
}

#[derive(Debug)]
enum ParsingError {
    Unknown,
}

impl FromStr for Pipes {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Pipes::Vertical),
            "-" => Ok(Pipes::Horizontal),
            "J" => Ok(Pipes::WestToNorth),
            "7" => Ok(Pipes::WestToSouth),
            "L" => Ok(Pipes::EastToNorth),
            "F" => Ok(Pipes::EastToSouth),
            "." => Ok(Pipes::NotAPipe),
            "S" => Ok(Pipes::Start),
            _ => Err(ParsingError::Unknown),
        }
    }
}

impl PipeMaze {
    fn get_pipe(&self, x: usize, y: usize) -> char {
        // println!("{}", self.map[x].chars().nth(y).unwrap());
        self.map[y].chars().nth(x).unwrap()
    }
    fn available_directions(&self, x: usize, y: usize) -> Vec<Directions> {
        // left
        let left_connector = if x == 0 {
            None
        } else {
            match self
                .get_pipe(x - 1, y)
                .to_string()
                .parse::<Pipes>()
                .unwrap()
            {
                Pipes::EastToNorth | Pipes::EastToSouth | Pipes::Horizontal => {
                    Some(Directions::Left)
                }
                _ => None,
            }
        };
        let right_connector = match self
            .get_pipe(x + 1, y)
            .to_string()
            .parse::<Pipes>()
            .unwrap()
        {
            Pipes::WestToNorth | Pipes::WestToSouth | Pipes::Horizontal => Some(Directions::Right),
            _ => None,
        };
        let top_connector = if y == 0 {
            None
        } else {
            match self
                .get_pipe(x, y - 1)
                .to_string()
                .parse::<Pipes>()
                .unwrap()
            {
                Pipes::EastToSouth | Pipes::WestToSouth | Pipes::Vertical => Some(Directions::Up),
                _ => None,
            }
        };
        let bot_connector = match self
            .get_pipe(x, y + 1)
            .to_string()
            .parse::<Pipes>()
            .unwrap()
        {
            Pipes::EastToNorth | Pipes::WestToNorth | Pipes::Vertical => Some(Directions::Down),
            _ => None,
        };
        let mut directions = vec![];
        for d in [
            left_connector,
            right_connector,
            top_connector,
            bot_connector,
        ]
        .into_iter()
        .flatten()
        // If None, gets ignored
        {
            directions.push(d)
        }
        directions
    }

    fn travel(&self, x: usize, y: usize, going: Directions) -> (usize, usize, Directions) {
        let x: i32 = x.try_into().unwrap();
        let y: i32 = y.try_into().unwrap();
        let (delta_x, delta_y): (i32, i32) = match going {
            Directions::Up => (0, -1),
            Directions::Down => (0, 1),
            Directions::Left => (-1, 0),
            Directions::Right => (1, 0),
            _ => unreachable!(),
        };
        let x = (x + delta_x).try_into().unwrap();
        let y = (y + delta_y).try_into().unwrap();
        let going = match (self.get_pipe(x, y).to_string().parse().unwrap(), going) {
            (Pipes::Start, _) => Directions::Arrived,
            (Pipes::Vertical, Directions::Up) => Directions::Up,
            (Pipes::WestToSouth, Directions::Up) => Directions::Left,
            (Pipes::EastToSouth, Directions::Up) => Directions::Right,
            (Pipes::WestToNorth, Directions::Right) => Directions::Up,
            (Pipes::WestToSouth, Directions::Right) => Directions::Down,
            (Pipes::Horizontal, Directions::Right) => Directions::Right,
            (Pipes::EastToNorth, Directions::Left) => Directions::Up,
            (Pipes::EastToSouth, Directions::Left) => Directions::Down,
            (Pipes::Horizontal, Directions::Left) => Directions::Left,
            (Pipes::Vertical, Directions::Down) => Directions::Down,
            (Pipes::EastToNorth, Directions::Down) => Directions::Right,
            (Pipes::WestToNorth, Directions::Down) => Directions::Left,
            _ => unreachable!(),
        };
        (x, y, going)
    }

    fn longest_distance_from_starting_position(&self) -> usize {
        let mut x = 0;
        let mut y = 0;
        for (i, line) in self.map.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == 'S' {
                    x = j;
                    y = i;
                    break;
                }
            }
        }

        let directions = self.available_directions(x, y);
        let mut next_direction = *directions.first().unwrap();
        let mut steps = 0;

        while next_direction != Directions::Arrived {
            // println!("{x} {y} {}", self.get_pipe(x, y));
            let (new_x, new_y, new_d) = self.travel(x, y, next_direction);
            x = new_x;
            y = new_y;
            next_direction = new_d;
            steps += 1;
        }

        steps / 2
    }

    fn new(map: Vec<String>) -> Self {
        Self { map }
    }
}

fn main() {
    let inputs = helpers::get_inputs();
    let m = PipeMaze::new(inputs);
    println!(
        "longest distance from starting position = {}",
        m.longest_distance_from_starting_position()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn maze_example_1() -> PipeMaze {
        let inputs = vec![
            ".....".to_string(),
            ".S-7.".to_string(),
            ".|.|.".to_string(),
            ".L-J.".to_string(),
            ".....".to_string(),
        ];

        PipeMaze::new(inputs)
    }

    #[test]
    fn distance_example1() {
        assert_eq!(
            maze_example_1().longest_distance_from_starting_position(),
            4
        );
    }
    #[test]
    fn distance_example2() {
        let inputs = vec![
            "..F7.".to_string(),
            ".FJ|.".to_string(),
            "SJ.L7".to_string(),
            "|F--J".to_string(),
            "LJ...".to_string(),
        ];

        let m = PipeMaze::new(inputs);
        assert_eq!(m.longest_distance_from_starting_position(), 8);
    }
    #[test]
    fn available_directions_example_1() {
        let directions = maze_example_1().available_directions(1, 1);
        assert!(directions.contains(&Directions::Down), "{directions:?}");
        assert!(directions.contains(&Directions::Right),);
        assert_eq!(directions.len(), 2,);
    }
}
