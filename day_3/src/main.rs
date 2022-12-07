use std::collections::HashSet;
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

fn part_one(input: &Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|line| {
            get_overlap(line)
                .iter()
                .map(|&c| get_priority(c))
                .sum::<u32>()
        })
        .sum()
}

fn part_two(input: &Vec<String>) -> u32 {
    input
        .chunks(3)
        .into_iter()
        .map(|lines| {
            lines.iter().fold(HashSet::new(), |acc, line| {
                if acc.is_empty() {
                    line.chars().collect()
                } else {
                    &acc & &line.chars().collect()
                }
            })
        })
        .map(|char_set| char_set.iter().map(|&c| get_priority(c)).sum::<u32>())
        .sum()
}

fn get_overlap(line: &str) -> HashSet<char> {
    let pivot = line.len() / 2;
    let (first, second) = (&line[0..pivot], &line[pivot..]);
    let first_set: HashSet<char> = first.chars().collect();

    second.chars().filter(|c| first_set.contains(c)).collect()
}

fn get_priority(c: char) -> u32 {
    let char_value = c as u32;
    if char_value < 'a' as u32 {
        char_value - 'A' as u32 + 27
    } else {
        char_value - 'a' as u32 + 1
    }
}
