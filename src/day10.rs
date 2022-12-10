use std::collections::VecDeque;

use crate::common::read_file;

#[derive(Debug)]
enum Op {
    Addx(u32, i32),
    Noop(u32),
}

struct CPU {
    reg: i32,
    cycle: u32,
    ops: VecDeque<Op>,
    crt: String,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            reg: 1,
            cycle: 0,
            ops: VecDeque::new(),
            crt: String::new(),
        }
    }

    fn addx(&mut self, value: i32) {
        self.reg += value;
    }

    fn noop(&self) {}

    fn one_cycle(&mut self) {
        match self.ops.pop_front() {
            Some(Op::Addx(2, n)) => self.ops.push_front(Op::Addx(1, n)),
            Some(Op::Addx(1, n)) => self.addx(n),
            Some(Op::Noop(1)) => self.noop(),
            _ => (),
        }
        self.cycle += 1;
    }

    fn add_op(&mut self, op: Op) {
        self.ops.push_back(op);
    }

    fn parse_op(&mut self, op: &str) {
        match op.trim().split(' ').collect::<Vec<&str>>()[..] {
            ["addx", value] => self.add_op(Op::Addx(2, value.parse::<i32>().unwrap())),
            ["noop"] => self.add_op(Op::Noop(1)),
            _ => panic!("Unknown operation"),
        }
    }

    fn draw_px(&mut self, cycle: u32, screen_length: u32) {
        let finished_cycle = (self.cycle as i32) - 1 - (cycle as i32 - screen_length as i32);
        if self.reg >= finished_cycle && self.reg - 3 < finished_cycle {
            self.crt.push('#');
        } else {
            self.crt.push('.');
        }
    }

    fn draw_eol(&mut self) {
        self.crt.push('\n');
    }
}

fn sum_at_cycles(data: Vec<String>, cycles: Vec<u32>) -> i32 {
    let mut cpu = CPU::new();
    let mut result = 0;

    data.iter().for_each(|line| cpu.parse_op(line));
    cpu.cycle += 1;

    for cycle in cycles {
        while cpu.cycle < cycle {
            cpu.one_cycle();
        }
        result += cpu.cycle as i32 * cpu.reg;
    }

    result
}

fn draw_at_screen(data: Vec<String>, screen_length: u32, screen_height: usize) -> String {
    let mut cpu = CPU::new();
    let cycles = Vec::from_iter((1..=screen_height).map(|height| screen_length * height as u32));

    cpu.draw_eol();
    data.iter().for_each(|line| cpu.parse_op(line));

    for cycle in cycles {
        while cpu.cycle < cycle {
            cpu.draw_px(cycle, screen_length);
            cpu.one_cycle();
        }
        cpu.draw_eol();
    }

    cpu.crt.clone()
}

pub fn run_part1(path: String) -> String {
    let data = read_file(path);
    let cycles = Vec::from_iter([20, 60, 100, 140, 180, 220]);
    sum_at_cycles(data, cycles).to_string()
}

pub fn run_part2(path: String) -> String {
    let data = read_file(path);
    draw_at_screen(data, 40, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_and_part2() {
        let data = vec![
            "addx 15".to_string(),
            "addx -11".to_string(),
            "addx 6".to_string(),
            "addx -3".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -8".to_string(),
            "addx 13".to_string(),
            "addx 4".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -35".to_string(),
            "addx 1".to_string(),
            "addx 24".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 16".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 21".to_string(),
            "addx -15".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -3".to_string(),
            "addx 9".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 8".to_string(),
            "addx 1".to_string(),
            "addx 5".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -36".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 6".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 7".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx 13".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx -33".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 8".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 17".to_string(),
            "addx -9".to_string(),
            "addx 1".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "addx 26".to_string(),
            "addx -30".to_string(),
            "addx 12".to_string(),
            "addx -1".to_string(),
            "addx 3".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -9".to_string(),
            "addx 18".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 9".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx -37".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "noop".to_string(),
            "addx 15".to_string(),
            "addx -21".to_string(),
            "addx 22".to_string(),
            "addx -6".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -10".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 20".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "addx 2".to_string(),
            "addx -6".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
        ];
        let cycles = Vec::from_iter([20, 60, 100, 140, 180, 220]);
        let result = sum_at_cycles(data.clone(), cycles);
        assert_eq!(result, 13140);

        let screen = draw_at_screen(data, 40, 6);
        println!("{}", screen);
    }
}
