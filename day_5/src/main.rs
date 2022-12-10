use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
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

    let (stacks, moves) = parse_input(&lines);

    part_one(&stacks, &moves);
    part_two(&stacks, &moves);
}

fn part_one(stacks: &HashMap<usize, VecDeque<char>>, moves: &Vec<Move>) {
    let mut stacks = stacks.clone();
    for mv in moves {
        mv.execute(&mut stacks);
    }

    for i in stacks.keys().sorted() {
        let top = stacks.get(i).unwrap().front().unwrap();
        print!("{}", top);
    }
    println!("");
}

fn part_two(stacks: &HashMap<usize, VecDeque<char>>, moves: &Vec<Move>) {
    let mut stacks = stacks.clone();
    for mv in moves {
        mv.execute_preserving_order(&mut stacks);
    }

    for i in stacks.keys().sorted() {
        let top = stacks.get(i).unwrap().front().unwrap();
        print!("{}", top);
    }
    println!("");
}

fn parse_input(input: &Vec<String>) -> (HashMap<usize, VecDeque<char>>, Vec<Move>) {
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();
    let mut moves: Vec<Move> = Vec::new();
    for line in input {
        if line.trim().starts_with('[') {
            for (i, c) in line.chars().enumerate() {
                if c.is_alphanumeric() {
                    let stack = i / 4 + 1;
                    stacks.entry(stack).or_insert(VecDeque::new()).push_back(c);
                }
            }
        } else if line.starts_with("move") {
            moves.push(Move::parse(line));
        }
    }

    (stacks, moves)
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn parse(s: &str) -> Move {
        let re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        let captures = re.captures(s).unwrap();

        let count = parse_match(captures.name("count"));
        let from = parse_match(captures.name("from"));
        let to = parse_match(captures.name("to"));

        Move { count, from, to }
    }

    fn execute(&self, stacks: &mut HashMap<usize, VecDeque<char>>) {
        for _ in 0..self.count {
            let from_stack = stacks.get_mut(&self.from).unwrap();
            let val = from_stack.pop_front().unwrap();
            let to_stack = stacks.get_mut(&self.to).unwrap();
            to_stack.push_front(val);
        }
    }

    fn execute_preserving_order(&self, stacks: &mut HashMap<usize, VecDeque<char>>) {
        let from_stack = stacks.get_mut(&self.from).unwrap();
        let mut vals = VecDeque::new();
        for _ in 0..self.count {
            let val = from_stack.pop_front().unwrap();
            vals.push_front(val);
        }
        let to_stack = stacks.get_mut(&self.to).unwrap();
        for val in vals {
            to_stack.push_front(val)
        }
    }
}

fn parse_match(m: Option<regex::Match>) -> usize {
    m.unwrap().as_str().parse().unwrap()
}
