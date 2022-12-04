use crate::common::read_file;

fn calc_calories(data: Vec<String>) -> Vec<i64> {
    let mut result = vec![0];
    for line in data {
        match line.parse::<i64>() {
            Ok(num) => *result.last_mut().unwrap() += num,
            Err(_) => result.push(0),
        }
    }
    result
}

pub fn run_part1() -> i64 {
    let data = read_file("data/day1.txt");
    let calories = calc_calories(data);
    *calories.iter().max().unwrap_or(&0)
}

pub fn run_part2() -> i64 {
    let data = read_file("data/day1.txt");
    let mut calories = calc_calories(data);
    calories.sort_by(|a, b| b.cmp(a));

    let mut result = 0;
    let mut numbers = 0;
    for calorie in calories {
        result += calorie;
        numbers += 1;
        if numbers >= 3 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        println!("{}", run_part1());
    }
}
