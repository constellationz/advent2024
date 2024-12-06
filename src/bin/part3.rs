use input::read_stdin;

#[path = "../input.rs"]
mod input;

enum Instruction {
    Multiply(i32, i32),
    Do,
    Dont,
    Unknown,
}

fn main() {
    let instructions = read_instructions(&read_stdin());
    println!(
        "result of multiplications: {}",
        eval_instructions(&instructions)
    );
}

fn eval_instructions(instructions: &[Instruction]) -> i32 {
    let mut mult = true;
    let mut sum = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Multiply(x, y) => {
                if mult {
                    sum += x * y
                }
            }
            Instruction::Do => mult = true,
            Instruction::Dont => mult = false,
            _ => (),
        }
    }
    sum
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
