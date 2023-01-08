use itertools::Itertools;

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn against(&self, opponent: &Self) -> Outcome {
        match self {
            Self::Rock => match opponent {
                Self::Scissors => Outcome::Win,
                Self::Paper => Outcome::Loss,
                Self::Rock => Outcome::Tie,
            },
            Self::Paper => match opponent {
                Self::Rock => Outcome::Win,
                Self::Scissors => Outcome::Loss,
                Self::Paper => Outcome::Tie,
            },
            Self::Scissors => match opponent {
                Self::Paper => Outcome::Win,
                Self::Rock => Outcome::Loss,
                Self::Scissors => Outcome::Tie,
            },
        }
    }
}

struct Turn {
    their_move: Move,
    my_move: Move,
}

impl Turn {
    fn new(pair: (char, char)) -> Option<Self> {
        let (their_move, my_move) = pair;
        Some(Self {
            their_move: match their_move {
                'A' => Move::Rock,
                'B' => Move::Paper,
                'C' => Move::Scissors,
                _ => return None,
            },
            my_move: match my_move {
                'X' => Move::Rock,
                'Y' => Move::Paper,
                'Z' => Move::Scissors,
                _ => return None,
            },
        })
    }

    fn my_score(&self) -> u32 {
        let move_score = match self.my_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let win_score = match self.my_move.against(&self.their_move) {
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Loss => 0,
        };

        return win_score + move_score;
    }
}

enum Outcome {
    Win,
    Loss,
    Tie,
}

/// Calculates the expected score
/// # Examples
/// ```
/// use std::fs;
/// use advent_of_code_2022_02::calculate_expected_score;
///
/// let contents = fs::read_to_string("example-input.txt").unwrap();
/// assert_eq!(calculate_expected_score(&contents), 15);
/// ```
pub fn calculate_expected_score(input: &String) -> u32 {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            Turn::new(
                x.split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| {
                        assert_eq!(x.len(), 1);
                        x.chars().nth(0).expect("There should be a char")
                    })
                    .collect_tuple()
                    .expect("There should be two items to put in the tuple"),
            )
            .expect("The Turn should build")
            .my_score()
        })
        .sum()
}
