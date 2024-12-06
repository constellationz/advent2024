use std::io::{stdin, Read};

pub fn read_stdin() -> String {
    let mut input: String = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    input
}
