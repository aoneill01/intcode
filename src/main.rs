use intcode::{Computer, util::read_program_from_file};

fn main() {
    let computer = Computer::new(read_program_from_file("day2.txt"));
    computer.step();
}
