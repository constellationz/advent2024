use input::read_stdin;

#[path = "../input.rs"]
mod input;

enum Instruction {
    Multiply(i32, i32),
    Do,
    Dont,
    Unknown,
}

struct ProgramState {
    mult: bool,
    sum: i32,
}

fn main() {
    let instructions = read_instructions(&read_stdin());
    println!(
        "result of multiplications: {}",
        ProgramState::new().eval_instructions(&instructions).sum
    );
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            mult: true,
            sum: 0,
        }
    }

    pub fn eval_instructions(&mut self, instructions: &[Instruction]) -> &ProgramState {
        for instruction in instructions {
            match instruction {
                Instruction::Multiply(x, y) => {
                    if self.mult {
                        self.sum += x * y
                    }
                }
                Instruction::Do => self.mult = true,
                Instruction::Dont => self.mult = false,
                _ => (),
            }
        }
        self
    }
}

fn read_instructions(input: &str) -> Vec<Instruction> {
    let re = regex::Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d+),(\d+)\))").unwrap();
    re.captures_iter(input).map(|re_match| {
        match &re_match[0] {
            "do()" => {
                Instruction::Do
            },
            "don't()" => {
                Instruction::Dont
            },
            complex => {
                if complex.contains("mul") {
                    Instruction::Multiply(
                        re_match[4].parse::<i32>().unwrap(),
                        re_match[5].parse::<i32>().unwrap()
                    )
                } else {
                    Instruction::Unknown
                }
            }
        }
    }).collect()
}
