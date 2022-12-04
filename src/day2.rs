use crate::common::read_file;
use core::panic;

fn read_file_and_split(path: &'static str) -> Vec<(String, String)> {
    read_file(path)
        .iter()
        .filter_map(|s| match s.split(" ").collect::<Vec<&str>>()[..] {
            [a, b] => Some((a.to_string(), b.to_string())),
            _ => None,
        })
        .collect::<Vec<(String, String)>>()
}

fn get_score(a: String, b: String) -> i32 {
    let mut score = 0;
    match b.as_str() {
        "X" => score += 1,
        "Y" => score += 2,
        "Z" => score += 3,
        _ => panic!("Unknown letter"),
    }
    match (a.as_str(), b.as_str()) {
        ("A", "X") | ("B", "Y") | ("C", "Z") => score += 3,
        ("A", "Y") | ("B", "Z") | ("C", "X") => score += 6,
        _ => (),
    }
    score
}

fn map_selected(data: Vec<(String, String)>) -> Vec<(String, String)> {
    data.into_iter()
        .filter_map(|(a, b)| match (a.as_str(), b.as_str()) {
            ("A", "X") => Some((String::from("A"), String::from("Z"))),
            ("A", "Y") => Some((String::from("A"), String::from("X"))),
            ("A", "Z") => Some((String::from("A"), String::from("Y"))),
            ("B", "X") => Some((String::from("B"), String::from("X"))),
            ("B", "Y") => Some((String::from("B"), String::from("Y"))),
            ("B", "Z") => Some((String::from("B"), String::from("Z"))),
            ("C", "X") => Some((String::from("C"), String::from("Y"))),
            ("C", "Y") => Some((String::from("C"), String::from("Z"))),
            ("C", "Z") => Some((String::from("C"), String::from("X"))),
            _ => None,
        })
        .collect::<Vec<(String, String)>>()
}

fn get_scores(data: Vec<(String, String)>) -> i32 {
    data.iter()
        .fold(0, |curr, (a, b)| curr + get_score(a.clone(), b.clone()))
}

pub fn run_part1() -> i32 {
    let data = read_file_and_split("./data/day2.txt");
    get_scores(data)
}

pub fn run_part2() -> i32 {
    let data = read_file_and_split("./data/day2.txt");
    get_scores(map_selected(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        let data = vec![
            (String::from("A"), String::from("Y")),
            (String::from("B"), String::from("X")),
            (String::from("C"), String::from("Z")),
        ];
        assert_eq!(get_scores(data), 15);
        assert_eq!(run_part1(), 12645);
    }

    #[test]
    fn test_day2_part2() {
        let data = vec![
            (String::from("A"), String::from("Y")),
            (String::from("B"), String::from("X")),
            (String::from("C"), String::from("Z")),
        ];
        assert_eq!(get_scores(map_selected(data)), 12);
        assert_eq!(run_part2(), 11756);
    }
}
