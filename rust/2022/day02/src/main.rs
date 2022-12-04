use std::{cmp::Ordering, str::FromStr, string::ParseError};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Move {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> std::option::Option<std::cmp::Ordering> {
        Some(if self == other {
            Ordering::Equal
        } else if self.greater() == *other {
            Ordering::Less
        } else {
            Ordering::Greater
        })
    }
}

impl Move {
    fn score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    /// return the variant greater than `self`
    fn greater(&self) -> Self {
        match self {
            Move::Rock => Self::Paper,
            Move::Paper => Self::Scissors,
            Move::Scissors => Self::Rock,
        }
    }

    /// return the variant less than `self`
    fn less(&self) -> Self {
        match self {
            Move::Rock => Self::Scissors,
            Move::Paper => Self::Rock,
            Move::Scissors => Self::Paper,
        }
    }
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => panic!("failed to match {s}"),
        }
    }
}

fn score(our: Move, their: Move) -> (usize, usize) {
    let (o, t) = if our == their {
        (3, 3)
    } else if our < their {
        (0, 6)
    } else {
        (6, 0)
    };
    (o + our.score(), t + their.score())
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => todo!(),
        }
    }
}

fn part1(s: &str) -> usize {
    let mut total_score = 0;
    for line in s.lines() {
        let sp: Vec<_> = line.split_ascii_whitespace().collect();
        let theirs = Move::from_str(sp[0]).unwrap();
        let ours = Move::from_str(sp[1]).unwrap();
        let score = score(ours, theirs);
        total_score += score.0;
    }

    total_score
}

fn part2(s: &str) -> usize {
    let mut total_score = 0;
    for line in s.lines() {
        let sp: Vec<_> = line.split_ascii_whitespace().collect();
        let theirs = Move::from_str(sp[0]).unwrap();
        let outcome = Outcome::from_str(sp[1]).unwrap();
        let ours = match (theirs, outcome) {
            (t, Outcome::Draw) => t,
            (t, Outcome::Win) => t.greater(),
            (t, Outcome::Lose) => t.less(),
        };
        let score = score(ours, theirs);
        total_score += score.0;
    }

    total_score
}

fn main() {
    let s = include_str!("input.txt");

    let p1 = part1(&s);
    let p2 = part2(&s);

    println!("{p1}");
    println!("{p2}");

    assert_eq!(p1, 13809);
    assert_eq!(p2, 12316);
}
