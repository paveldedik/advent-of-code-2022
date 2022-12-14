use advent_of_code::day1;
use advent_of_code::day10;
use advent_of_code::day11;
use advent_of_code::day12;
use advent_of_code::day2;
use advent_of_code::day3;
use advent_of_code::day4;
use advent_of_code::day5;
use advent_of_code::day6;
use advent_of_code::day7;
use advent_of_code::day8;
use advent_of_code::day9;

type RunPart = fn(String) -> String;

fn main() {
    let days: Vec<(&str, RunPart, RunPart)> = vec![
        ("day1", day1::run_part1, day1::run_part2),
        ("day2", day2::run_part1, day2::run_part2),
        ("day3", day3::run_part1, day3::run_part2),
        ("day4", day4::run_part1, day4::run_part2),
        ("day5", day5::run_part1, day5::run_part2),
        ("day6", day6::run_part1, day6::run_part2),
        ("day7", day7::run_part1, day7::run_part2),
        ("day8", day8::run_part1, day8::run_part2),
        ("day9", day9::run_part1, day9::run_part2),
        ("day10", day10::run_part1, day10::run_part2),
        ("day11", day11::run_part1, day11::run_part2),
        ("day12", day12::run_part1, day12::run_part2),
    ];
    for (name, part1, part2) in days {
        let path = format!("data/{name}.txt");
        println!(
            "{name} result part1: {}, part2: {}",
            part1(path.clone()),
            part2(path)
        )
    }
}
