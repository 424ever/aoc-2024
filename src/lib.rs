use std::fs;

pub fn read_input(day: u32) -> String {
    let path = format!("inputs/day{day}");
    fs::read_to_string(path).unwrap()
}
