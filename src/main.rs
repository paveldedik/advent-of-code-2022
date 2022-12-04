use advent_of_code::day1;
use advent_of_code::day2;
use advent_of_code::day3;
use advent_of_code::day4;

type RunPart = fn(String) -> i64;

fn main() {
    let days: Vec<(&'static str, RunPart, RunPart)> = vec![
        ("day1", day1::run_part1, day1::run_part2),
        ("day2", day2::run_part1, day2::run_part2),
        ("day3", day3::run_part1, day3::run_part2),
        ("day4", day4::run_part1, day4::run_part2),
    ];
    for (name, part1, part2) in days {
        let path = format!("data/{}.txt", name);
        println!("{} result part1: {}, part2: {}", name, part1(path.clone()), part2(path))
    }
}
