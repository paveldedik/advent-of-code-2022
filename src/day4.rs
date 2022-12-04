use crate::common::read_file;
use std::collections::HashSet;

fn parse_range(range: String) -> (i32, i32) {
    match range.split("-").collect::<Vec<&str>>()[..] {
        [n1, n2] => (n1.parse().unwrap(), n2.parse().unwrap()),
        _ => panic!("Wrong format"),
    }
}

fn expand_range(range: String) -> HashSet<i32> {
    let (r1, r2) = parse_range(range);
    (r1..=r2).collect::<HashSet<i32>>()
}

fn range_contains_another(range1: (i32, i32), range2: (i32, i32)) -> bool {
    let (left1, right1) = range1;
    let (left2, right2) = range2;
    if left1 >= left2 && right1 <= right2 {
        return true;
    } else if left2 >= left1 && right2 <= right1 {
        return true;
    } else {
        return false;
    }
}

fn ranges_overlap(range1: HashSet<i32>, range2: HashSet<i32>) -> bool {
    match range1.intersection(&range2).next() {
        Some(_) => return true,
        None => match range2.intersection(&range1).next() {
            Some(_) => return true,
            None => return false,
        },
    }
}

fn count_containing_matches(data: Vec<String>) -> i32 {
    let mut result = 0;
    data.iter()
        .for_each(|pair| match pair.split(",").collect::<Vec<&str>>()[..] {
            [r1, r2] => {
                if range_contains_another(parse_range(r1.to_string()), parse_range(r2.to_string()))
                {
                    result += 1;
                }
            }
            _ => (),
        });
    result
}

fn count_overlaping_matches(data: Vec<String>) -> i32 {
    let mut result = 0;
    data.iter()
        .for_each(|pair| match pair.split(",").collect::<Vec<&str>>()[..] {
            [r1, r2] => {
                if ranges_overlap(expand_range(r1.to_string()), expand_range(r2.to_string())) {
                    result += 1;
                }
            }
            _ => (),
        });
    result
}

pub fn run_part1() -> i32 {
    let data = read_file("data/day4.txt");
    count_containing_matches(data)
}

pub fn run_part2() -> i32 {
    let data = read_file("data/day4.txt");
    count_overlaping_matches(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(count_containing_matches(data), 2);
    }

    #[test]
    fn test_part2() {
        let data = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(count_overlaping_matches(data), 4);
    }
}
