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
    get_sorted_elf_totals(input).into_iter()
        .take(1)
        .sum()
}

fn part_two(input: &Vec<String>) -> i32 {
    get_sorted_elf_totals(input).into_iter()
        .take(3)
        .sum()
}

fn get_sorted_elf_totals(input: &Vec<String>) -> Vec<i32> {
    let mut elf_totals: Vec<i32> = Vec::new();
    let mut sub_total: i32 = 0;
    for line in input {
        if line.trim().is_empty() {
            elf_totals.push(sub_total);
            sub_total = 0;
        } else {
            sub_total += line.parse::<i32>().unwrap();
        }
    }

    elf_totals.sort();
    elf_totals.reverse(); // Descending order
    return elf_totals
}
