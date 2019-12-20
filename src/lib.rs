use std::fs;

pub struct Computer {
    program: Vec<i32>,
    program_counter: usize
}

impl Computer {
    pub fn new(program: Vec<i32>) -> Computer {
        Computer {
            program,
            program_counter: 0
        }
    }

    pub fn step(&self) {
        println!("In step, pc: {}, mem.size: {}", self.program_counter, self.program.len());
    }
}

pub fn read_program_from_file(filename: &str) -> Vec<i32> {
    let data = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    data.split(",")
        .map(|val| val.parse::<i32>().unwrap())
        .collect()
}