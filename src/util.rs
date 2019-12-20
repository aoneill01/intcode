use std::fs;

pub fn read_program_from_file(filename: &str) -> Vec<i32> {
    let data = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    data.split(",")
        .map(|val| val.parse::<i32>().unwrap())
        .collect()
}