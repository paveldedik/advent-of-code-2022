use std::collections::HashMap;

use crate::common::read_file;

struct Matrix {
    grid: Vec<Vec<u32>>,
    start: (usize, usize),
    end: Vec<(usize, usize)>,
}

fn parse_matrix(data: Vec<String>) -> Matrix {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let grid = data
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| match ch {
                    'S' => {
                        start = (i, j);
                        1
                    }
                    'E' => {
                        end = (i, j);
                        26
                    }
                    ch => {
                        let n: u32 = ch.into();
                        n - 96
                    }
                })
                .collect::<Vec<u32>>()
        })
        .collect();
    Matrix { grid, start, end: vec![end] }
}

fn invert_matrix(mut matrix: Matrix) -> Matrix {
    let mut start = (0, 0);
    let mut end = Vec::new();

    for i in 0..matrix.grid.len() {
        for j in 0..matrix.grid[i].len() {
            if i == matrix.end[0].0 && j == matrix.end[0].1 {
                start = (i, j);
            } else if matrix.grid[i][j] == 1 {
                end.push((i, j));
            }
            matrix.grid[i][j] = 27 - matrix.grid[i][j];
        }
    }

    matrix.start = start;
    matrix.end = end;
    matrix
}

fn get_surrounding(grid: &Vec<Vec<u32>>, path: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if let Some((pos_i, pos_j)) = path.last().cloned() {
        for (i, j) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            if pos_i == 0 && i == -1
                || pos_j == 0 && j == -1
                || pos_i == grid.len() - 1 && i == 1
                || pos_j == grid[pos_i].len() - 1 && j == 1
            {
                continue;
            }
            let (new_i, new_j) = ((pos_i as i32 + i) as usize, (pos_j as i32 + j) as usize);
            if path.contains(&(new_i, new_j)) {
                continue;
            }
            if grid[pos_i][pos_j] + 1 >= grid[new_i][new_j]
            {
                result.push((new_i, new_j));
            }
        }
    }
    result
}

fn get_id(path: &Vec<(usize, usize)>) -> i64 {
    let mut hashcode = 0;
    for (idx, (pos_i, pos_j)) in path.iter().enumerate() {
        let number = idx as i64 + 1;
        hashcode += number * (pos_i + 1) as i64 * 3 + number * (pos_j + 1) as i64 * 11;
    }
    hashcode * path.len() as i64
}

#[allow(dead_code)]
fn print_path_in_matrix(matrix: &Matrix, path: &Vec<(usize, usize)>) {
    for i in 0..matrix.grid.len() {
        for j in 0..matrix.grid[i].len() {
            if path.contains(&(i, j)) {
                print!("■");
            } else {
                let n = matrix.grid[i][j] as u8 + 96;
                let ch: char = n.into();
                print!("{}", ch);
            }
        }
        println!();
    }
    println!()
}

fn find_shortest_paths(matrix: Matrix) -> Option<Vec<(usize, usize)>> {
    let mut path = vec![matrix.start.clone()];
    let mut found_paths = Vec::new();
    let mut branches = HashMap::new();
    let mut visited = HashMap::new();
    let mut backtracking = false;

    while let Some(item) = path.last() {
        if matrix.end.contains(&item) {
            found_paths.push(path.clone());
        }
        let previous_path_len = visited.get(item).copied().unwrap_or(10000);
        let next = if backtracking || previous_path_len > path.len() {
            visited.insert(item.clone(), path.len());
            let path_id = get_id(&path);
            match branches.get_mut(&path_id) {
                None => {
                    let surrounding = get_surrounding(&matrix.grid, &path);
                    branches.insert(path_id.clone(), surrounding);
                    branches.get_mut(&path_id).unwrap().pop()
                }
                Some(surrounding) => surrounding.pop(),
            }
        } else {
            None
        };
        backtracking = next.is_none();
        match next {
            None => _ = path.pop(),
            Some(item) => path.push(item),
        }
    }

    let min_length = found_paths.iter().map(|path| path.len()).min().unwrap_or(0);
    found_paths
        .iter()
        .filter(|path| path.len() == min_length)
        .next()
        .cloned()
}

fn get_number_of_steps(path: Option<Vec<(usize, usize)>>) -> u32 {
    match path {
        Some(p) => (p.len() as i32 - 1) as u32,
        None => 0,
    }
}

pub fn run_part1(path: String) -> String {
    let matrix = parse_matrix(read_file(path));
    get_number_of_steps(find_shortest_paths(matrix)).to_string()
}

pub fn run_part2(path: String) -> String {
    let matrix = parse_matrix(read_file(path));
    let inverted_matrix = invert_matrix(matrix);
    get_number_of_steps(find_shortest_paths(inverted_matrix)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<String> {
        vec![
            "Sabqponm".to_string(),
            "abcryxxl".to_string(),
            "accszExk".to_string(),
            "acctuvwj".to_string(),
            "abdefghi".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        let matrix = parse_matrix(get_test_data());
        let shortest_path = find_shortest_paths(matrix);
        assert_eq!(get_number_of_steps(shortest_path), 31);
    }

    #[test]
    fn test_part2() {
        let matrix = invert_matrix(parse_matrix(get_test_data()));
        let shortest_path = find_shortest_paths(matrix);
        assert_eq!(get_number_of_steps(shortest_path), 29);
    }
}
