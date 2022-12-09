use std::iter::repeat;

use crate::common::read_file;

enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

type Playground = Vec<Vec<u8>>;
type Position = (usize, usize);

fn init_playground(size: usize) -> Playground {
    Vec::from_iter((0..size).map(|_| vec![0; size]))
}

#[allow(dead_code)]
fn print_playground(playground: &Playground, tail: &[Position]) {
    for i in 0..playground.len() {
        let mut line = String::new();
        for j in 0..playground[i].len() {
            let mut rope = false;
            for (idx, (x, y)) in tail.iter().enumerate() {
                if i == *x && j == *y {
                    rope = true;
                    if idx > 0 {
                        line.push_str(idx.to_string().as_str())
                    } else {
                        line.push('H')
                    }
                    break;
                }
            }
            if !rope {
                if playground[i][j] == 1 {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
        }
        println!("{}", line);
    }
    println!();
}

fn parse_line(line: String) -> (Instruction, usize) {
    match line.split(' ').collect::<Vec<&str>>()[..] {
        [instruction, steps] => {
            let steps_num = steps.parse::<usize>().unwrap();
            match instruction {
                "U" => (Instruction::Up, steps_num),
                "D" => (Instruction::Down, steps_num),
                "L" => (Instruction::Left, steps_num),
                "R" => (Instruction::Right, steps_num),
                _ => panic!("Unknown instruction"),
            }
        }
        _ => panic!("Unknown command"),
    }
}

fn calculate_tail_position(head: &Position, tail: &Position) -> Position {
    match (head.0 as i16 - tail.0 as i16, head.1 as i16 - tail.1 as i16) {
        // .....    .....    .....    .....
        // .....    .....    ..H..    ..H..
        // ...H. -> ..TH. or ..... -> ..T..
        // .T...    .....    .T...    .....
        // .....    .....    .....    .....
        (2, -1) | (1, -2) => (tail.0 + 1, tail.1 - 1),
        // .....    .....
        // ..H..    ..H..
        // ..... -> ..T..
        // ..T..    .....
        // .....    .....
        (0, -2) => (tail.0, tail.1 - 1),
        // .....    .....    .....    .....
        // .....    .....    ..H..    ..H..
        // .H... -> .HT.. or ..... -> ..T..
        // ...T.    .....    ...T.    .....
        // .....    .....    .....    .....
        (-2, -1) | (-1, -2) => (tail.0 - 1, tail.1 - 1),
        // .....    .....
        // .....    .....
        // .H.T. -> .HT..
        // .....    .....
        // .....    .....
        (-2, 0) => (tail.0 - 1, tail.1),
        // .....    .....    .....    .....
        // ...T.    .....    ...T.    .....
        // .H... -> .HT.. or ..... -> ..T..
        // .....    .....    ..H..    ..H..
        // .....    .....    .....    .....
        (-2, 1) | (-1, 2) => (tail.0 - 1, tail.1 + 1),
        // .....    .....
        // ..T..    .....
        // ..... -> ..T..
        // ..H..    ..H..
        // .....    .....
        (0, 2) => (tail.0, tail.1 + 1),
        // .....    .....    .....    .....
        // .T...    .....    .T...    .....
        // ..... -> ..T.. or ...H. -> ..TH.
        // ..H..    ..H..    .....    .....
        // .....    .....    .....    .....
        (2, 1) | (1, 2) => (tail.0 + 1, tail.1 + 1),
        // .....    .....
        // .....    .....
        // .T.H. -> ..TH.
        // .....    .....
        // .....    .....
        (2, 0) => (tail.0 + 1, tail.1),
        // cases where tail moves diagonally
        (2, 2) => (tail.0 + 1, tail.1 + 1),
        (-2, 2) => (tail.0 - 1, tail.1 + 1),
        (2, -2) => (tail.0 + 1, tail.1 - 1),
        (-2, -2) => (tail.0 - 1, tail.1 - 1),
        // rest of the cases don't require any change
        _ => *tail,
    }
}

fn move_head(
    playground: &mut Playground,
    instruction: &Instruction,
    steps: &usize,
    tail: &mut Vec<Position>,
) {
    (0..*steps).for_each(|_| {
        match instruction {
            Instruction::Up => tail[0].0 -= 1,
            Instruction::Down => tail[0].0 += 1,
            Instruction::Left => tail[0].1 -= 1,
            Instruction::Right => tail[0].1 += 1,
        }
        (0..(tail.len() - 1)).for_each(|i| {
            tail[i + 1] = calculate_tail_position(&tail[i], &tail[i + 1]);
        });
        playground[tail[tail.len() - 1].0][tail[tail.len() - 1].1] = 1;
    })
}

fn process_instructions(data: Vec<String>, mut playground: Playground, length: usize) -> u32 {
    let (cols, rows) = (playground.len(), playground[0].len());
    let mut tail = Vec::from_iter(repeat((cols / 2, rows / 2)).take(length));
    playground[tail[0].0][tail[0].1] = 1;

    for line in data {
        let (instruction, steps) = parse_line(line);
        move_head(&mut playground, &instruction, &steps, &mut tail);
    }
    playground
        .iter()
        .map(|col| col.iter().map(|&item| item as u32).sum::<u32>())
        .sum()
}

pub fn run_part1(path: String) -> String {
    let data = read_file(path);
    let playground = init_playground(1000);
    process_instructions(data, playground, 2).to_string()
}

pub fn run_part2(path: String) -> String {
    let data = read_file(path);
    let playground = init_playground(1000);
    process_instructions(data, playground, 10).to_string()
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
        let playground = init_playground(20);
        assert_eq!(process_instructions(data, playground, 2), 13);
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
        let playground = init_playground(50);
        assert_eq!(process_instructions(data, playground, 10), 36);
    }
}
