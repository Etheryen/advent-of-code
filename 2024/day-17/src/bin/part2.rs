use std::time::Instant;

#[derive(Debug, Clone)]
struct Computer {
    program: Vec<u64>,
    instruction_pointer: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    output: Vec<u64>,
}

impl Computer {
    fn run(&mut self) -> bool {
        while self.instruction_pointer < self.program.len() {
            // println!("out: {:?}, program: {:?}", self.output, self.program);
            if self.output != self.program[0..self.output.len()] {
                return false;
            }

            let instruction = self.program[self.instruction_pointer];

            let operand = self.program[self.instruction_pointer + 1];

            let mut should_increment_ip = true;

            match instruction {
                0 => self.reg_a >>= self.combo(operand),
                1 => self.reg_b ^= operand,
                2 => self.reg_b = self.combo(operand) % 8,
                3 => {
                    if self.reg_a != 0 {
                        self.instruction_pointer = operand as usize;
                        should_increment_ip = false;
                    }
                }
                4 => self.reg_b ^= self.reg_c,
                5 => self.output.push(self.combo(operand) % 8),
                6 => self.reg_b = self.reg_a >> self.combo(operand),
                7 => self.reg_c = self.reg_a >> self.combo(operand),
                _ => unreachable!(),
            };

            if should_increment_ip {
                self.instruction_pointer += 2;
            }
        }

        self.output == self.program
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
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

// program: 2,4, 1,1, 7,5, 1,5, 4,2, 5,5, 0,3, 3,0
// 2,4) B = A % 8
// 1,1) B = (A % 8) ^ 1
// 7,5) C = A >> ((A % 8) ^ 1)
// 1,5) B = ((A % 8) ^ 1) ^ 5
// 4,2) B = (((A % 8) ^ 1) ^ 5) ^ (A >> ((A % 8) ^ 1))
//
// 5,5) OUT_1 <- ((((A % 8) ^ 1) ^ 5) ^ (A >> ((A % 8) ^ 1))) % 8
// 0,3) A = A >> 3;
// 3,0) IF A != 0 GOTO 2,4
//
// OUT_1 == 2
//
// OUT == ((((A % 8) ^ 1) ^ 5) ^ (A >> ((A % 8) ^ 1))) % 8
// because its always octal
// OUT == (((A % 8) ^ 1) ^ 5) ^ (A >> ((A % 8) ^ 1))
//
//  XXX
// ^100

fn to_out_num(reg_a: u8) -> u8 {
    assert!((0..=7).contains(&reg_a));
    ((reg_a ^ 1) ^ 5) ^ (reg_a >> (reg_a ^ 1))
}

fn u8_vec_to_u64_num(octals: Vec<u8>) -> u64 {
    let mut result = 0;

    for octal in octals {
        result <<= 3;
        result += octal as u64;
    }

    result
}

fn solve(lines: Vec<&str>) -> u64 {
    let computer = to_computer(lines);

    let mut out_octals = Vec::new();

    for program_num in computer.program.iter() {
        for input_num in 0..=7 {
            let out_num = to_out_num(input_num);

            if out_num as u64 == *program_num {
                out_octals.push(input_num);
                break;
            }
        }
    }

    u8_vec_to_u64_num(out_octals)
}

fn main() {
    let lines = include_str!("input.txt").lines().collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
