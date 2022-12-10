use std::{fmt, iter::repeat};

use crate::common::read_file;

type Position = (usize, usize);

struct Playground {
    data: Vec<Vec<u8>>,
    knots: Vec<Position>,
}

impl Playground {
    fn new(size: usize, number_of_knots: usize) -> Playground {
        let mut data = Vec::from_iter((0..size).map(|_| vec![0; size]));
        let knot_init: Position = (data.len() / 2, data[0].len() / 2);
        let knots: Vec<Position> = Vec::from_iter(repeat(knot_init).take(number_of_knots));
        data[knots[0].0][knots[0].1] = 1;
        Playground { data, knots }
    }

    fn update_tail_position(&mut self, head_idx: usize, tail_idx: usize) {
        let head = self.knots[head_idx];
        let tail = self.knots[tail_idx];
        let direction = (head.0 as i16 - tail.0 as i16, head.1 as i16 - tail.1 as i16);

        self.knots[tail_idx] = match direction {
            // ...    ... | .T.    ... | ...    ... | .H.    .H.
            // T.H -> .TH | ... -> .T. | H.T -> HT. | ... -> .T.
            // ...    ... | .H.    .H. | ...    ... | .T.    ...
            (2, 0) => (tail.0 + 1, tail.1),
            (0, 2) => (tail.0, tail.1 + 1),
            (-2, 0) => (tail.0 - 1, tail.1),
            (0, -2) => (tail.0, tail.1 - 1),

            // ...    ... | .H.    .H. | ..H    ..H
            // ..H -> .TH | ... -> .T. | ... -> .T.
            // T..    ... | T..    ... | T..    ...
            // ------------------------------------
            // ...    ... | .H.    .H. | H..    H..
            // H.. -> HT. | ... -> .T. | ... -> .T.
            // ..T    ... | ..T    ... | ..T    ...
            // ------------------------------------
            // ..T    ... | ..T    ... | ..T    ...
            // H.. -> HT. | ... -> .T. | ... -> .T.
            // ...    ... | .H.    .H. | H..    ...
            // ------------------------------------
            // T..    ... | T..    ... | T..    ...
            // ... -> .T. | ..H -> .TH | ... -> .T.
            // .H.    .H. | ...    ... | ..H    ..H
            (2, -1) | (1, -2) | (2, -2) => (tail.0 + 1, tail.1 - 1),
            (-2, -1) | (-1, -2) | (-2, -2) => (tail.0 - 1, tail.1 - 1),
            (-2, 1) | (-1, 2) | (-2, 2) => (tail.0 - 1, tail.1 + 1),
            (2, 1) | (1, 2) | (2, 2) => (tail.0 + 1, tail.1 + 1),

            // rest of the cases don't require any change
            _ => tail,
        }
    }

    fn update_head_position(&mut self, instruction: &str) {
        let head = self.knots[0];
        self.knots[0] = match instruction {
            "U" => (head.0 - 1, head.1),
            "D" => (head.0 + 1, head.1),
            "L" => (head.0, head.1 - 1),
            "R" => (head.0, head.1 + 1),
            _ => head,
        }
    }

    fn move_knots(&mut self, instruction: String, steps: usize) {
        for _ in 0..steps {
            self.update_head_position(&instruction);
            for i in 1..self.knots.len() {
                self.update_tail_position(i - 1, i);
            }
            let last_tail = self.knots[self.knots.len() - 1];
            self.data[last_tail.0][last_tail.1] = 1;
        }
    }

    fn parse_line(&self, line: String) -> (String, usize) {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            [instruction, steps] => (instruction.to_string(), steps.parse::<usize>().unwrap()),
            _ => panic!("Unknown command"),
        }
    }

    fn load_vec(&mut self, data: Vec<String>) {
        for line in data {
            let (instruction, steps) = self.parse_line(line);
            self.move_knots(instruction, steps);
        }
    }

    fn sum(&self) -> u32 {
        self.data
            .iter()
            .map(|col| col.iter().map(|&item| item as u32).sum::<u32>())
            .sum()
    }
}

impl fmt::Display for Playground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data = String::new();
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                let mut rope = false;
                for (idx, (x, y)) in self.knots.iter().enumerate() {
                    if i == *x && j == *y {
                        rope = true;
                        if idx > 0 {
                            data.push_str(idx.to_string().as_str());
                        } else {
                            data.push('H');
                        }
                        break;
                    }
                }
                if !rope {
                    if self.data[i][j] == 1 {
                        data.push('#');
                    } else {
                        data.push('.');
                    }
                }
            }
            data.push('\n');
        }
        write!(f, "{}", data)
    }
}

pub fn run_part1(path: String) -> String {
    let data = read_file(path);
    let mut playground = Playground::new(1000, 2);
    playground.load_vec(data);
    playground.sum().to_string()
}

pub fn run_part2(path: String) -> String {
    let data = read_file(path);
    let mut playground = Playground::new(1000, 10);
    playground.load_vec(data);
    playground.sum().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = vec![
            "R 4".to_string(),
            "U 4".to_string(),
            "L 3".to_string(),
            "D 1".to_string(),
            "R 4".to_string(),
            "D 1".to_string(),
            "L 5".to_string(),
            "R 2".to_string(),
        ];
        let mut playground = Playground::new(15, 2);
        playground.load_vec(data);

        println!("{}", &playground);
        assert_eq!(playground.sum(), 13);
    }

    #[test]
    fn test_part2() {
        let data = vec![
            "R 5".to_string(),
            "U 8".to_string(),
            "L 8".to_string(),
            "D 3".to_string(),
            "R 17".to_string(),
            "D 10".to_string(),
            "L 25".to_string(),
            "U 20".to_string(),
        ];
        let mut playground = Playground::new(30, 10);
        playground.load_vec(data);

        println!("{}", &playground);
        assert_eq!(playground.sum(), 36);
    }
}
