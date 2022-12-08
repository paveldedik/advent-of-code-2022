use crate::common::read_file;

fn parse_data(data: Vec<String>) -> Vec<Vec<u32>> {
    data.iter()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn is_visible(matrix: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    if i == 0 || j == 0 || i == matrix.len() - 1 || j == matrix[i].len() - 1 {
        return true;
    }

    let current = matrix[i][j];
    matrix[..i].iter().map(|line| line[j]).max().unwrap() < current
        || matrix[i + 1..].iter().map(|line| line[j]).max().unwrap() < current
        || *matrix[i][..j].iter().max().unwrap() < current
        || *matrix[i][j + 1..].iter().max().unwrap() < current
}

fn get_distance(trees: &mut impl Iterator<Item = u32>, current: u32) -> u32 {
    let mut distance = 0;
    for tree in trees.by_ref() {
        distance += 1;
        if tree >= current {
            break;
        }
    }
    distance
}

fn get_viewing_distance(matrix: &[Vec<u32>], i: usize, j: usize) -> u32 {
    let current = matrix[i][j];
    get_distance(&mut matrix[..i].iter().rev().map(|line| line[j]), current)
        * get_distance(&mut matrix[i + 1..].iter().map(|line| line[j]), current)
        * get_distance(&mut matrix[i][..j].iter().rev().copied(), current)
        * get_distance(&mut matrix[i][j + 1..].iter().copied(), current)
}

fn count_visible(matrix: Vec<Vec<u32>>) -> u32 {
    let mut visibility_counter = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if is_visible(&matrix, i, j) {
                visibility_counter += 1;
            }
        }
    }
    visibility_counter
}

fn get_max_viewing_distance(matrix: Vec<Vec<u32>>) -> u32 {
    let mut max_distance = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let viewing_distance = get_viewing_distance(&matrix, i, j);
            if viewing_distance > max_distance {
                max_distance = viewing_distance;
            }
        }
    }
    max_distance
}

pub fn run_part1(path: String) -> String {
    let data = read_file(path);
    let matrix = parse_data(data);
    count_visible(matrix).to_string()
}

pub fn run_part2(path: String) -> String {
    let data = read_file(path);
    let matrix = parse_data(data);
    get_max_viewing_distance(matrix).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];
        let matrix = parse_data(data);
        assert_eq!(count_visible(matrix), 21);
    }

    #[test]
    fn test_part2() {
        let data = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];
        let matrix = parse_data(data);
        assert_eq!(get_max_viewing_distance(matrix), 8);
    }
}
