use regex::Regex;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::{Rc, Weak};

use std::collections::HashMap;

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
    let root = Rc::new(RefCell::new(Directory {
        name: "/",
        parent: None,
        files: Vec::new(),
        subdirs: HashMap::new(),
    }));
    let mut pwd: Rc<RefCell<Directory>> = root.clone();
    for line in input {
        if line.starts_with("$") {
            let command = Command::parse(line);
            pwd = match command {
                Command::CDSub { target } => Directory::new(target, Some(pwd.clone())),
                Command::CDParent => pwd.borrow().get_parent().expect("Can't navigate above /"),
                Command::CDRoot => root.clone(),
                Command::LS => pwd,
            };
        } else {
            if line.starts_with("dir") {
                let re = Regex::new(r"dir (?P<name>.*)").unwrap();
                let captures = re.captures(line).unwrap();
                let name = captures.name("name").unwrap().as_str();

                Directory::new(name, Some(pwd.clone()));
            } else {
                pwd.borrow_mut().files.push(FileStruct::parse(line));
            }
        }
    }

    return root.borrow().get_size_with_limit(100000);
}

struct Directory<'a> {
    name: &'a str,
    parent: Option<Weak<RefCell<Directory<'a>>>>,
    files: Vec<FileStruct<'a>>,
    subdirs: HashMap<&'a str, Rc<RefCell<Directory<'a>>>>,
}

impl<'a> Directory<'a> {
    fn new(
        name: &'a str,
        parent: Option<Rc<RefCell<Directory<'a>>>>,
    ) -> Rc<RefCell<Directory<'a>>> {
        let parent_ref = match parent {
            Some(ref parent) => Some(Rc::downgrade(&(parent.clone()))),
            None => None,
        };
        let new_dir = Directory {
            name,
            parent: parent_ref,
            files: Vec::new(),
            subdirs: HashMap::new(),
        };
        let new_dir_ref = Rc::new(RefCell::new(new_dir));
        if let Some(parent) = parent {
            parent
                .borrow_mut()
                .subdirs
                .insert(name, new_dir_ref.clone());
        }
        new_dir_ref
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Directory<'a>>>> {
        self.parent.as_ref()?.clone().upgrade()
    }

    fn get_size(&self) -> usize {
        let self_size: usize = self.files.iter().map(|file| file.size).sum();
        let sub_dirs_size: usize = self
            .subdirs
            .values()
            .map(|dir| dir.borrow().get_size())
            .sum();
        self_size + sub_dirs_size
    }

    fn get_size_with_limit(&self, max_size: usize) -> usize {
        let self_size: usize = self.files.iter().map(|file| file.size).sum();
        let sub_dirs_size: usize = self
            .subdirs
            .values()
            .map(|dir| dir.borrow().get_size())
            .sum();
        let true_size = self_size + sub_dirs_size;
        let adjusted_size = if true_size <= max_size { true_size } else { 0 };
        adjusted_size
            + self
                .subdirs
                .values()
                .map(|dir| dir.borrow().get_size_with_limit(max_size))
                .sum::<usize>()
    }
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
    CDSub { target: &'a str },
    CDParent,
    CDRoot,
    LS,
}

impl<'a> Command<'a> {
    fn parse(input: &'a str) -> Command<'a> {
        if input == "$ cd /" {
            return Command::CDRoot;
        }
        if input == "$ cd .." {
            return Command::CDParent;
        }
        if input == "$ ls" {
            return Command::LS;
        }

        let re = Regex::new(r"\$ cd (?P<target>.*)").unwrap();
        let captures = re.captures(input).unwrap();
        let target = captures.name("target").unwrap().as_str();

        Command::CDSub { target }
    }
}
