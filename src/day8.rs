#[derive(Copy, Clone)]
enum Instruction {
    acc(i64),
    nop(i64),
    jmp(i64),
}

struct Computer {
    acc: i64,
    pc: i64,
    instructions: Vec<(Instruction, bool)>,
    call_stack: Vec<i64>
}

impl Computer {
    fn new() -> Self {
        Computer {
            acc: 0,
            pc: 0,
            instructions: vec![],
            call_stack: vec![]
        }
    }

    fn load_program(&mut self, raw: &str) {
        for line in raw.lines() {
            let mut vals = line.split(' ');
            match vals.next().unwrap() {
                "nop" => self.instructions.push((Instruction::nop(vals.next().unwrap().parse().unwrap()), false)),
                "acc" => self.instructions.push((Instruction::acc(vals.next().unwrap().parse().unwrap()), false)),
                "jmp" => self.instructions.push((Instruction::jmp(vals.next().unwrap().parse().unwrap()), false)),
                _ => {},
            }
        }
    }

    fn reset(&mut self) {
        for instruction in self.instructions.iter_mut() {
            instruction.1 = false;
        }
        self.pc = 0;
        self.acc = 0;
    }

    fn run_program(&mut self) -> bool {
        while (self.pc as usize) < self.instructions.len() {
            let (instruction, has_run) = self.instructions[self.pc as usize];
            if has_run {
                return false;
            }

            self.instructions[self.pc as usize].1 = true;
            self.call_stack.push(self.pc);

            match instruction {
                Instruction::acc(inc) => {
                    self.acc += inc;
                    self.pc += 1;
                }, 
                Instruction::nop(_) => {
                    self.pc += 1;
                }
                Instruction::jmp(inc) => {
                    self.pc += inc;
                }
            }
        }
        true
    }

    fn flip_inst(&mut self, index: usize) {
        match self.instructions[index].0 {
            Instruction::acc(_) => {}, 
            Instruction::nop(val) => {
                self.instructions[index].0 = Instruction::jmp(val);
            }, 
            Instruction::jmp(val) => {
                self.instructions[index].0 = Instruction::nop(val);
            }, 
        }
    }

    fn find_fault(&mut self) {
        self.run_program();
        let mut og_callstack = self.call_stack.clone();

        loop {
            self.reset();
            let pc = og_callstack.pop().unwrap();
            
            self.flip_inst(pc as usize);

            if self.run_program() {
                return;
            }

            self.flip_inst(pc as usize);
        }
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let mut comp = Computer::new();
    comp.load_program(input);
    comp.run_program();
    comp.acc
}

pub fn puzzle2(input: &str) -> i64 {
    let mut comp = Computer::new();
    comp.load_program(input);
    comp.find_fault();
    comp.acc
}
