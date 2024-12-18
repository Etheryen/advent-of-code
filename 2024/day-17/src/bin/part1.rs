use std::time::Instant;

#[derive(Debug)]
struct Computer {
    program: Vec<u8>,
    instruction_pointer: usize,
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    output: Vec<u32>,
}

impl Computer {
    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            let instruction = self.program[self.instruction_pointer];
            println!("instruction: {}", instruction);

            let operand = self.program[self.instruction_pointer + 1];

            let mut should_increment_ip = true;

            match instruction {
                0 => self.reg_a /= 2_u32.pow(self.combo(operand)),
                1 => self.reg_b ^= operand as u32,
                2 => self.reg_b = self.combo(operand) % 8,
                3 => {
                    if self.reg_a != 0 {
                        self.instruction_pointer = operand as usize;
                        should_increment_ip = false;
                    }
                }
                4 => self.reg_b ^= self.reg_c,
                5 => self.output.push(self.combo(operand) % 8),
                6 => self.reg_b = self.reg_a / 2_u32.pow(self.combo(operand)),
                7 => self.reg_c = self.reg_a / 2_u32.pow(self.combo(operand)),
                _ => unreachable!(),
            };

            if should_increment_ip {
                self.instruction_pointer += 2;
            }
        }
    }

    fn combo(&self, operand: u8) -> u32 {
        match operand {
            0..=3 => operand as u32,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }
}

fn to_computer(lines: Vec<&str>) -> Computer {
    let program = lines[4]
        .split_once(' ')
        .unwrap()
        .1
        .split(',')
        .map(|instruction| instruction.parse().unwrap())
        .collect();

    let reg_a = lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let reg_b = lines[1].split_once(": ").unwrap().1.parse().unwrap();
    let reg_c = lines[2].split_once(": ").unwrap().1.parse().unwrap();

    Computer {
        program,
        instruction_pointer: 0,
        reg_a,
        reg_b,
        reg_c,
        output: Vec::new(),
    }
}

fn solve(lines: Vec<&str>) -> String {
    let mut computer = to_computer(lines);
    computer.run();
    println!(
        "reg_a: {}, reg_b: {}, reg_c: {}",
        computer.reg_a, computer.reg_b, computer.reg_c
    );

    computer
        .output
        .into_iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let lines = include_str!("input.txt").lines().collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
