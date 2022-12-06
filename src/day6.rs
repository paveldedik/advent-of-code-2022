use std::collections::HashSet;

use crate::common::read_file;

fn read_stream(path: String) -> String {
    let data = read_file(path);
    data.first().unwrap().clone()
}

fn get_marker(stream: String, length: usize) -> Option<usize> {
    let mut current: Vec<char> = Vec::new();
    for (idx, ch) in stream.chars().into_iter().enumerate() {
        current.push(ch);
        if current.len() > length {
            current.remove(0);
        }
        if HashSet::<_>::from_iter(&current).len() == length {
            return Some(idx + 1);
        }
    }
    None
}

pub fn run_part1(path: String) -> String {
    let stream = read_stream(path);
    get_marker(stream, 4).unwrap().to_string()
}

pub fn run_part2(path: String) -> String {
    let stream = read_stream(path);
    get_marker(stream, 14).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(get_marker(input, 4), Some(5));
    }

    #[test]
    fn test_part2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(get_marker(input, 14), Some(19));
    }
}
