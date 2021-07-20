mod urd;

use std::io;
use urd::{Mode, convert};

fn switch_mode(mode: Mode) -> Mode {
    match mode {
        Mode::ToUrd => {
            println!("Entering FROMURD mode. use ! to switch to TOURD mode.");
            Mode::FromUrd
        },
        Mode::FromUrd => {
            println!("Entering TOURD mode. use ! to switch to FROMURD mode.");
            Mode::ToUrd
        },
    }
}

fn main() {
    let mut mode = switch_mode(Mode::FromUrd);
    println!("Use Q to quit the urdifier.");
    
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        match &input[..] {
            "!" => {
                mode = switch_mode(mode);
            },
            "Q" => break,
            _ => {
                println!("{}", convert(input, &mode));
            }
        };
    }
}