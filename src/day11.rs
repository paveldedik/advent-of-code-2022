use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

use regex::Regex;

use crate::common::read_file_split_by;

fn parse_monkey_data(input: String) -> HashMap<String, String> {
    let regex = Regex::new(r"Monkey (?P<id>\d+):\n +Starting items: (?P<items>[\d, ]+)\n +Operation: new = (?P<op_arg1>[\w\d]+) (?P<op_fun>[+*]) (?P<op_arg2>[\w\d]+)\n +Test: divisible by (?P<test_division>\d+)\n   +If true: throw to monkey (?P<if_division_true>\d+)\n   +If false: throw to monkey (?P<if_division_false>\d+)").unwrap();
    let caps = regex.captures(&input).unwrap();
    regex
        .capture_names()
        .flatten()
        .filter_map(|n| Some((n.to_string(), caps.name(n)?.as_str().to_string())))
        .collect::<HashMap<String, String>>()
}

fn get_operation(op_fun: String, op_arg1: String, op_arg2: String) -> Box<dyn Fn(u32) -> u32> {
    let fun = match op_fun.as_str() {
        "+" => |arg1, arg2| arg1 + arg2,
        "*" => |arg1, arg2| arg1 * arg2,
        _ => panic!("Unknown operation"),
    };
    Box::new(move |worry_level| {
        let arg1 = match op_arg1.as_str() {
            "old" => worry_level,
            num => num.parse::<u32>().unwrap(),
        };
        let arg2 = match op_arg2.as_str() {
            "old" => worry_level,
            num => num.parse::<u32>().unwrap(),
        };
        fun(arg1, arg2)
    })
}

fn get_test(
    test_division: String,
    if_division_true: String,
    if_division_false: String,
) -> Box<dyn Fn(u32) -> u32> {
    let division = test_division.parse::<u32>().unwrap();
    let monkey_if_true = if_division_true.parse::<u32>().unwrap();
    let monkey_if_false = if_division_false.parse::<u32>().unwrap();
    Box::new(move |worry_level| {
        if worry_level % division == 0 {
            monkey_if_true
        } else {
            monkey_if_false
        }
    })
}

struct Monkey {
    id: u32,
    items: VecDeque<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> u32>,
}

impl Monkey {
    fn from_string(input: String) -> Self {
        let data = parse_monkey_data(input);

        let id = data["id"].parse::<u32>().unwrap();
        let items = data["items"]
            .split(", ")
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<VecDeque<u32>>();
        let operation = get_operation(
            data["op_fun"].clone(),
            data["op_arg1"].clone(),
            data["op_arg2"].clone(),
        );
        let test = get_test(
            data["test_division"].clone(),
            data["if_division_true"].clone(),
            data["if_division_false"].clone(),
        );

        Self {
            id,
            items,
            operation,
            test,
        }
    }

    fn play(&mut self) -> Option<(u32, u32)> {
        match self.items.pop_front() {
            None => None,
            Some(mut worry_level) => {
                worry_level = (self.operation)(worry_level);
                worry_level /= 3;
                Some(((self.test)(worry_level), worry_level))
            }
        }
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkey {} has {:?}", self.id, self.items)
    }
}

fn let_monkeys_play(data: Vec<String>, rounds: u32) -> u32 {
    let mut monkeys: HashMap<u32, Monkey> = HashMap::from_iter(data.iter().map(|input| {
        let monkey = Monkey::from_string(input.trim().to_string());
        (monkey.id, monkey)
    }));
    let mut inspect_counter: HashMap<u32, u32> =
        HashMap::from_iter(monkeys.keys().map(|monkey_id| (*monkey_id, 0)));

    for _ in 0..rounds {
        for monkey_id in 0..(monkeys.len() as u32) {
            while let Some((throw_monkey_id, worry_level)) =
                monkeys.get_mut(&monkey_id).unwrap().play()
            {
                *inspect_counter.get_mut(&monkey_id).unwrap() += 1;
                let monkey = monkeys.get_mut(&throw_monkey_id).unwrap();
                monkey.items.push_back(worry_level);
            }
        }
    }

    let mut inspections = inspect_counter.values().copied().collect::<Vec<u32>>();
    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

pub fn run_part1(path: String) -> String {
    let data = read_file_split_by(path, "\n\n");
    let_monkeys_play(data, 20).to_string()
}

pub fn run_part2(_: String) -> String {
    "not implemented".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<String> {
        vec![
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
                .to_string(),
            "Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0"
                .to_string(),
            "Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3"
                .to_string(),
            "Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
                .to_string(),
        ]
    }

    #[test]
    fn test_parsing_to_monkey() {
        let data = String::from(
            "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3",
        );
        let mut monkey = Monkey::from_string(data);
        assert_eq!(monkey.play(), Some((3, 500)));
        assert_eq!(monkey.play(), Some((3, 620)));
        assert_eq!(monkey.play(), None);
    }

    #[test]
    fn test_part1() {
        let result = let_monkeys_play(get_test_data(), 20);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part2() {}
}
