use std::fs;

pub mod containers;
pub mod iters;

pub fn read_input(name: &str) -> String {
    let path = format!("inputs/{name}");
    fs::read_to_string(path).unwrap()
}
