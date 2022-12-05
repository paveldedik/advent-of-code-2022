use crate::common::read_file;
use std::collections::HashSet;

fn read_file_maybe_split(path: String, split_in_middle: bool) -> Vec<String> {
    let mut result = Vec::new();
    read_file(path).iter().for_each(|line| {
        if line.is_empty() {
            return;
        }
        if split_in_middle {
            let (first, last) = line.split_at(line.len() / 2);
            result.push(String::from(first));
            result.push(String::from(last));
        } else {
            result.push(String::from(line));
        }
    });
    result
}

fn char_to_int(ch: char) -> u32 {
    let shift = if ch.is_lowercase() { 96 } else { 38 };
    let n: u32 = ch.into();
    n - shift
}

fn find_common(data: &[String]) -> char {
    let mut letters: HashSet<char> = HashSet::from_iter(data[0].chars());
    data.iter().for_each(|line| {
        letters = HashSet::from_iter(
            letters
                .intersection(&HashSet::from_iter(line.chars()))
                .copied()
        );
    });
    *letters.iter().next().unwrap()
}

fn find_commons(data: Vec<Vec<String>>) -> i64 {
    let mut result = 0;
    data.iter()
        .for_each(|group| result += char_to_int(find_common(group)));
    result as i64
}

fn group_by(data: Vec<String>, groups: usize) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    for i in 0..(data.len() / groups) {
        result.push(Vec::new());
        for j in 0..groups {
            let pos = (i * groups) + j;
            if data.len() > pos {
                result[i].push(data[pos].clone());
            }
        }
        if result[result.len() - 1].is_empty() {
            result.remove(result.len() - 1);
        }
    }
    result
}

pub fn run_part1(path: String) -> String {
    let data = read_file_maybe_split(path, true);
    find_commons(group_by(data, 2)).to_string()
}

pub fn run_part2(path: String) -> String {
    let data = read_file_maybe_split(path, false);
    find_commons(group_by(data, 3)).to_string()
}
