use std::collections::HashMap;

pub mod util;

pub struct Computer {
    instruction_handlers: HashMap<i32, Box<dyn Fn() -> ()>>,
    program: Vec<i32>,
    program_counter: usize
}

impl Computer {
    pub fn new(program: Vec<i32>) -> Computer {
        let result = Computer {
            instruction_handlers: HashMap::new(),
            program,
            program_counter: 0
        };

        // result.init_instruction_handlers();

        result
    }

    pub fn step(&self) {
        println!("In step, pc: {}, mem.size: {}, inst.size: {}", 
            self.program_counter, self.program.len(), self.instruction_handlers.len());
    }

    // fn add(&self) {

    // }

    // fn init_instruction_handlers(&self) {
    //     self.instruction_handlers.insert(1, Box::new(|| self.add()));
    // }
}
