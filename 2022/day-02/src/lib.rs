use std::fs::read;

use itertools::Itertools;

#[derive(Clone, PartialEq, Eq)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    pub fn into_score(self) -> u32 {
        self as u32
    }

    pub fn get_sup(&self) -> Move {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn get_inf(&self) -> Move {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "a" | "x" => Move::Rock,
            "b" | "y" => Move::Paper,
            "c" | "z" => Move::Scissors,
            _ => panic!("invalid move"),
        }
    }
}

pub enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl From<&str> for RoundResult {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "x" => Self::Loss,
            "y" => Self::Draw,
            "z" => Self::Win,
            _ => panic!("invalid move"),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;

        if self.eq(other) {
            return Some(Equal);
        }

        if self.get_sup().eq(other) {
            return Some(Less);
        }

        Some(Greater)
    }
}

pub fn load_mm_rounds(s: &str) -> Vec<(Move, Move)> {
    s.lines().filter(|line| !line.is_empty()).map(|line| {
        line.split(' ').map(Move::from).collect_tuple().unwrap()
    }).collect_vec()
}

pub fn load_mr_rounds(s: &str) -> Vec<(Move, RoundResult)> {
    s.lines().filter(|line| !line.is_empty()).map(|line| {
        let ss = line.split(' ').collect::<Vec<_>>();
        (Move::from(ss[0]), RoundResult::from(ss[1]))
    }).collect_vec()
}

pub fn load_file(filename: &str) -> String {
    let bytes = read(filename).unwrap();

    String::from_utf8(bytes).unwrap()
}

