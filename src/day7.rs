use std::{collections::HashMap, error::Error, fmt};

use regex::Regex;

use crate::common::read_file;

#[derive(Debug)]
pub struct FileSystemError(String);

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for FileSystemError {}

struct Content {
    #[allow(dead_code)]
    name: String,
    content: HashMap<String, Content>,
    size: Option<i32>,
}

impl Content {
    fn new(name: String) -> Content {
        Content {
            name,
            content: HashMap::new(),
            size: None,
        }
    }

    fn new_file(name: String, bytes: i32) -> Content {
        let mut content = Content::new(name);
        content.size = Some(bytes);
        content
    }

    fn is_file(&self) -> bool {
        self.size.is_some()
    }

    fn count_bytes(&self) -> i32 {
        match self.size {
            Some(size) => size,
            None => {
                let mut result = 0;
                for content in self.content.values() {
                    result += content.count_bytes();
                }
                result
            }
        }
    }

    fn iter_dirs(&self) -> impl Iterator<Item = &Self> {
        self.content.values().filter(|entity| !entity.is_file())
    }
}

pub struct FileSystem {
    root: Content,
    cwd: Vec<String>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            root: Content::new("root".to_string()),
            cwd: Vec::new(),
        }
    }

    fn open(&mut self) -> &mut Content {
        let mut dir = &mut self.root;
        for dir_name in &self.cwd {
            dir = dir.content.get_mut(dir_name).unwrap();
        }
        dir
    }

    pub fn cd(&mut self, name: String) -> Result<(), FileSystemError> {
        match name.as_str() {
            ".." => {
                self.cwd.pop();
                return Ok(());
            }
            "/" => {
                self.cwd.clear();
                return Ok(());
            }
            _ => (),
        }

        let dir = self.open();
        match dir.content.get(&name) {
            Some(content) if !content.is_file() => {
                self.cwd.push(name);
                Ok(())
            }
            Some(_) => Err(FileSystemError(
                "Given argument is not a directory".to_string(),
            )),
            None => Err(FileSystemError("Directory not found".to_string())),
        }
    }

    pub fn mkdir(&mut self, name: String) -> Result<(), FileSystemError> {
        let dir = self.open();
        match dir.content.get(&name) {
            Some(_) => Err(FileSystemError(
                "File or directory already exists".to_string(),
            )),
            None => {
                dir.content.insert(name.clone(), Content::new(name));
                Ok(())
            }
        }
    }

    pub fn write_bytes(&mut self, name: String, bytes: i32) -> Result<(), FileSystemError> {
        let dir = self.open();
        match dir.content.get(&name) {
            Some(_) => Err(FileSystemError(
                "File or directory already exists".to_string(),
            )),
            None => {
                dir.content
                    .insert(name.clone(), Content::new_file(name, bytes));
                Ok(())
            }
        }
    }
}

fn re_match(regex: &str, text: &str, matches: Option<usize>) -> Option<Vec<String>> {
    let re = Regex::new(regex).unwrap();
    if re.is_match(text) {
        match matches {
            Some(n) => {
                let captures = re.captures(text).unwrap();
                Some(
                    (1..=n)
                        .map(|idx| {
                            captures
                                .get(idx)
                                .map_or("".to_string(), |m| m.as_str().to_string())
                        })
                        .collect(),
                )
            }
            _ => Some(Vec::new()),
        }
    } else {
        None
    }
}

fn parse_command(text: &str) -> Option<(&'static str, Vec<String>)> {
    if let Some(args) = re_match(r"\$ cd (.*)", text, Some(1)) {
        Some(("cd", args))
    } else if re_match(r"\$ ls", text, None).is_some() {
        None
    } else if let Some(args) = re_match(r"dir (.*)", text, Some(1)) {
        Some(("dir", args))
    } else if let Some(args) = re_match(r"(\d+) (.*)", text, Some(2)) {
        Some(("file", args))
    } else {
        None
    }
}

fn process_actions(data: Vec<String>) -> FileSystem {
    let mut fs = FileSystem::new();
    for line in data {
        match parse_command(&line) {
            Some(("cd", args)) if args.len() == 1 => _ = fs.cd(args[0].clone()),
            Some(("dir", args)) if args.len() == 1 => _ = fs.mkdir(args[0].clone()),
            Some(("file", args)) if args.len() == 2 => {
                _ = fs.write_bytes(args[1].clone(), args[0].clone().parse::<i32>().unwrap())
            }
            None => (),
            _ => println!("err: Unknown command {}", line),
        }
    }
    fs
}

fn count_size(dir: &Content, max_size: Option<i32>) -> i32 {
    let mut stack = vec![dir];
    let mut result = 0;
    while let Some(dir) = stack.pop() {
        let bytes = dir.count_bytes();
        if max_size.is_none() || bytes <= max_size.unwrap() {
            result += bytes;
        }
        dir.iter_dirs().for_each(|dir| stack.push(dir))
    }
    result
}

fn find_freeable_space(dir: &Content, min_size: i32) -> i32 {
    let mut stack = vec![dir];
    let mut result = Vec::new();
    while let Some(dir) = stack.pop() {
        let bytes = dir.count_bytes();
        if bytes >= min_size {
            result.push(bytes);
        }
        dir.iter_dirs().for_each(|dir| stack.push(dir))
    }
    *result.iter().min().unwrap()
}

pub fn run_part1(path: String) -> String {
    let data = read_file(path);
    let fs = process_actions(data);
    count_size(&fs.root, Some(100000)).to_string()
}

pub fn run_part2(path: String) -> String {
    let data = read_file(path);
    let fs = process_actions(data);
    let needed_space = 30000000 - (70000000 - fs.root.count_bytes());
    find_freeable_space(&fs.root, needed_space).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];
        let fs = process_actions(data.clone());
        let result = count_size(&fs.root, Some(100000));
        assert_eq!(result, 95437);

        let fs = process_actions(data);
        let needed_space = 30000000 - (70000000 - fs.root.count_bytes());
        let freeable_space = find_freeable_space(&fs.root, needed_space);
        assert_eq!(freeable_space, 24933642);
    }
}
