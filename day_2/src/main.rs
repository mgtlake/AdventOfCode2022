use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("Could not read file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Line could not be read"))
        .collect();

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}

fn part_one(input: &Vec<String>) -> i32 {
    let mut total_score = 0;
    for line in input {
        let moves = line.trim().split(" ").collect::<Vec<&str>>();
        let their_move = Move::parse(moves.get(0).unwrap());
        let my_move = Move::parse(moves.get(1).unwrap());
        total_score += score_round(&my_move, &their_move);
    }

    total_score
}

fn part_two(input: &Vec<String>) -> i32 {
    let mut total_score = 0;
    for line in input {
        let moves = line.trim().split(" ").collect::<Vec<&str>>();
        let their_move = Move::parse(moves.get(0).unwrap());
        let desired_outcome = Outcome::parse(moves.get(1).unwrap());

        let my_move = desired_outcome.get_move_for_outcome(&their_move);
        total_score += score_round(&my_move, &their_move);
    }

    total_score
}

#[derive(PartialEq, Copy, Clone)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

enum Outcome {
    WIN,
    LOSS,
    DRAW,
}

impl Outcome {
    fn get_score(&self) -> i32 {
        match self {
            Outcome::WIN => 6,
            Outcome::DRAW => 3,
            Outcome::LOSS => 0,
        }
    }

    fn parse(symbol: &str) -> Self {
        match symbol {
            "X" => Outcome::LOSS,
            "Y" => Outcome::DRAW,
            "Z" => Outcome::WIN,
            _ => panic!("Invalid outcome"),
        }
    }

    fn get_move_for_outcome(&self, their_move: &Move) -> Move {
        match self {
            Outcome::WIN => their_move.loses_to(),
            Outcome::DRAW => their_move.clone(),
            Outcome::LOSS => their_move.wins_to(),
        }
    }
}

impl Move {
    fn parse(symbol: &str) -> Self {
        match symbol {
            "A" | "X" => Move::ROCK,
            "B" | "Y" => Move::PAPER,
            "C" | "Z" => Move::SCISSORS,
            _ => panic!("Invalid move"),
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            Move::ROCK => 1,
            Move::PAPER => 2,
            Move::SCISSORS => 3,
        }
    }

    fn wins_to(&self) -> Move {
        match self {
            Move::ROCK => Move::SCISSORS,
            Move::PAPER => Move::ROCK,
            Move::SCISSORS => Move::PAPER,
        }
    }

    fn loses_to(&self) -> Move {
        match self {
            Move::ROCK => Move::PAPER,
            Move::PAPER => Move::SCISSORS,
            Move::SCISSORS => Move::ROCK,
        }
    }

    fn get_outcome(&self, other: &Move) -> Outcome {
        if *self == other.loses_to() {
            Outcome::WIN
        } else if *self == other.wins_to() {
            Outcome::LOSS
        } else {
            Outcome::DRAW
        }
    }
}

fn score_round(your_move: &Move, their_move: &Move) -> i32 {
    your_move.get_score() + your_move.get_outcome(their_move).get_score()
}
