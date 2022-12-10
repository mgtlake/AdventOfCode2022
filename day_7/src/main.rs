use regex::Regex;
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
}

fn part_one(input: &Vec<String>) -> usize {
    let mut root = Directory {
        name: "/",
        parent: None,
        files: Vec::new(),
        subdirs: Vec::new(),
    };
    let mut pwd = root;
    for line in input {
        if line.starts_with("$") {
            let command = Command::parse(line);
            let mut next_pwd: Directory = match command {
                Command::CD_SUB { target } => {
                    let mut new_dir = Directory {
                        name: target,
                        parent: Some(&pwd),
                        files: Vec::new(),
                        subdirs: Vec::new(),
                    };
                    pwd.subdirs.push(&new_dir);
                    new_dir
                }
                // Command::CD_PARENT => *pwd.parent.as_ref().expect("Can't navigate above /"),
                // Command::CD_ROOT => &mut root,
                // Command::LS => pwd,
                // _ => pwd,
            };
            // pwd = next_pwd;
        } else {
        }
    }
    0
}

struct Directory<'a> {
    name: &'a str,
    parent: Option<&'a Directory<'a>>,
    files: Vec<&'a FileStruct<'a>>,
    subdirs: Vec<&'a Directory<'a>>,
}

struct FileStruct<'a> {
    name: &'a str,
    size: usize,
}

impl<'a> FileStruct<'a> {
    fn parse(input: &'a str) -> FileStruct<'a> {
        let re = Regex::new(r"(?P<size>\d+) (?P<name>.*)").unwrap();
        let captures = re.captures(input).unwrap();
        let size = captures.name("size").unwrap().as_str().parse().unwrap();
        let name = captures.name("name").unwrap().as_str();

        FileStruct { name, size }
    }
}

enum Command<'a> {
    CD_SUB { target: &'a str },
    CD_PARENT,
    CD_ROOT,
    LS,
}

impl<'a> Command<'a> {
    fn parse(input: &'a str) -> Command<'a> {
        if input == "$ cd /" {
            return Command::CD_ROOT;
        }
        if input == "$ cd .." {
            return Command::CD_PARENT;
        }
        if input == "$ ls" {
            return Command::LS;
        }

        let re = Regex::new(r"\$ cd (?P<target>.*)").unwrap();
        let captures = re.captures(input).unwrap();
        let target = captures.name("target").unwrap().as_str();

        Command::CD_SUB { target }
    }
}
