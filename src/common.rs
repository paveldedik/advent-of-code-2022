use std::fs;

pub(crate) fn read_file_split_by(path: String, split_by: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(split_by)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub(crate) fn read_file(path: String) -> Vec<String> {
    read_file_split_by(path, "\n")
}
