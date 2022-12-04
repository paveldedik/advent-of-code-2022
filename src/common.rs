use std::fs;

pub(crate) fn read_file(path: &'static str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
