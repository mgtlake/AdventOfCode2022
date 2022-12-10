use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("Could not read file");
    let mut input = String::new();
    BufReader::new(file)
        .read_line(&mut input)
        .expect("Line could not be read");

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &String) -> usize {
    find_marker(input, 4).unwrap()
}

fn part_two(input: &String) -> usize {
    find_marker(input, 14).unwrap()
}

fn find_marker(input: &String, marker_size: usize) -> Option<usize> {
    for (i, chars) in input
        .chars()
        .collect::<Vec<char>>()
        .windows(marker_size)
        .enumerate()
    {
        if chars.iter().unique().count() == marker_size {
            return Some(i + marker_size);
        }
    }
    None
}
