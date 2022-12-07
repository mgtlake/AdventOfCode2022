use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("Could not read file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Line could not be read"))
        .collect();

    let ranges = lines.iter().map(|line| parse_range(line)).collect();

    println!("{}", part_one(&ranges));
    println!("{}", part_two(&ranges));
}

fn part_one(ranges: &Vec<(Range<i32>, Range<i32>)>) -> u32 {
    ranges
        .into_iter()
        .map(|(first, second)| (dominates(&first, &second) || dominates(&second, &first)) as u32)
        .sum()
}

fn part_two(ranges: &Vec<(Range<i32>, Range<i32>)>) -> u32 {
    ranges
        .into_iter()
        .map(|(first, second)| overlaps(&first, &second) as u32)
        .sum()
}

fn parse_range(s: &str) -> (Range<i32>, Range<i32>) {
    let re = Regex::new(
        r"(?P<first_start>\d+)-(?P<first_end>\d+),(?P<second_start>\d+)-(?P<second_end>\d+)",
    )
    .unwrap();
    let captures = re.captures(s).unwrap();
    let first_start = parse_match(captures.name("first_start"));
    let first_end = parse_match(captures.name("first_end"));
    let second_start = parse_match(captures.name("second_start"));
    let second_end = parse_match(captures.name("second_end"));
    (first_start..first_end, second_start..second_end)
}

fn parse_match(m: Option<regex::Match>) -> i32 {
    m.unwrap().as_str().parse().unwrap()
}

fn dominates<T: Ord>(me: &Range<T>, other: &Range<T>) -> bool {
    me.start <= other.start && other.end <= me.end
}

fn overlaps<T: Ord>(me: &Range<T>, other: &Range<T>) -> bool {
    me.start <= other.end && other.start <= me.end
}
