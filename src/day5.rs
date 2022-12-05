use crate::common::read_file;
use regex::Regex;
use std::fmt;

struct Cargo {
    stacks: Vec<Vec<char>>,
}

impl Cargo {
    fn new(size: usize) -> Self {
        let mut cargo = Cargo { stacks: Vec::new() };
        (0..size).for_each(|_| cargo.stacks.push(Vec::new()));
        cargo
    }

    fn add_crate(&mut self, pos: usize, load: char) {
        if load != ' ' {
            self.stacks[pos].push(load);
        }
    }

    fn pop_crate(&mut self, pos: usize) -> Option<char> {
        self.stacks[pos].pop()
    }

    fn move_crates(&mut self, crates: i32, from: usize, to: usize, keep_order: bool) {
        let mut stack = Vec::new();
        (0..crates).for_each(|_| {
            if let Some(crate_) = self.pop_crate(from) {
                stack.push(crate_)
            }
        });

        if keep_order {
            stack
                .iter()
                .rev()
                .for_each(|crate_| self.add_crate(to, *crate_))
        } else {
            stack.iter().for_each(|crate_| self.add_crate(to, *crate_))
        }
    }

    fn process_input(&mut self, line: String, keep_order: bool) {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        if let Some(caps) = re.captures(line.as_str()) {
            self.move_crates(
                caps[1].parse::<i32>().unwrap(),
                caps[2].parse::<usize>().unwrap() - 1,
                caps[3].parse::<usize>().unwrap() - 1,
                keep_order,
            )
        }
    }

    fn get_top(&mut self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack[stack.len() - 1])
            .collect::<String>()
    }
}

impl fmt::Display for Cargo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data = String::new();
        for (pos, stack) in self.stacks.iter().enumerate() {
            data += &format!("{pos}: {}\n", stack.iter().collect::<String>());
        }
        write!(f, "{}", data)
    }
}

fn init_cargo(data: Vec<String>) -> Cargo {
    let re = Regex::new(r"(?:\[(?P<crate>\w)\]|(?:( )   ))").unwrap();
    let mut size = 0;
    let mut height = 0;

    for line in &data {
        if re.is_match(line.as_str()) {
            height += 1;
        } else {
            size = line
                .split(' ')
                .filter_map(|num| num.parse::<usize>().ok())
                .max()
                .unwrap();
            break;
        }
    }
    let mut cargo = Cargo::new(size);

    for line in data[..height].iter().rev() {
        re.captures_iter(line.as_str())
            .enumerate()
            .for_each(|(pos, caps)| {
                caps.iter()
                    .filter_map(|m| match m {
                        Some(load) if load.as_str().len() == 1 => Some(load.as_str()),
                        _ => None,
                    })
                    .for_each(|load| cargo.add_crate(pos, load.chars().next().unwrap()));
            });
    }
    cargo
}

fn load_cargo(cargo: &mut Cargo, data: Vec<String>, keep_order: bool) {
    for line in data {
        cargo.process_input(line, keep_order);
    }
}

pub fn run_part1(path: String) -> String {
    let data = read_file(path);
    let mut cargo = init_cargo(data.clone());
    load_cargo(&mut cargo, data, false);
    cargo.get_top()
}

pub fn run_part2(path: String) -> String {
    let data = read_file(path);
    let mut cargo = init_cargo(data.clone());
    load_cargo(&mut cargo, data, true);
    cargo.get_top()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo() {
        let data = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        let mut cargo = init_cargo(data.clone());
        load_cargo(&mut cargo, data.clone(), false);
        assert_eq!(cargo.get_top(), "CMZ".to_string());

        let mut cargo = init_cargo(data.clone());
        load_cargo(&mut cargo, data.clone(), true);
        assert_eq!(cargo.get_top(), "MCD".to_string());
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            run_part1("data/day5.txt".to_string()),
            "QGTHFZBHV".to_string()
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            run_part2("data/day5.txt".to_string()),
            "MGDMPSZTM".to_string()
        )
    }
}
