use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("B: {}", file_path);
    let source_string = match fs::read_to_string(file_path) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };
    let source: Vec<char> = source_string.chars().collect();
    let mut mem_tape: Vec<u8> = vec![0; 30000];
    execute(source, &mut mem_tape, 0)
}

fn execute(source: Vec<char>, mem_tape: &mut Vec<u8>, initial_pointer_pos: usize) {
    let mut loop_index_stack: Vec<usize> = Vec::new();
    let mut code_index: usize = 0;

    let mut pointer: usize = initial_pointer_pos;
    while code_index < source.len() {
        match source[code_index] {
            '+' => mem_tape[pointer] = mem_tape[pointer].wrapping_add(1),
            '-' => mem_tape[pointer] = mem_tape[pointer].wrapping_sub(1),
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '.' => print!("{}", mem_tape[pointer] as char),
            ',' => mem_tape[pointer] = read_char(),
            '[' => match mem_tape[pointer] {
                0 => {
                    //move pointer to index of associated closing bracket
                    let mut open_bracket_counter = 0;
                    while open_bracket_counter >= 0 {
                        code_index += 1;
                        match source[code_index] {
                            '[' => open_bracket_counter += 1,
                            ']' => open_bracket_counter -= 1,
                            _ => (),
                        }
                    }
                }
                _ => loop_index_stack.push(code_index),
            },
            ']' => match mem_tape[pointer] {
                0 => _ = loop_index_stack.pop(),
                _ => {
                    code_index = *loop_index_stack
                        .last()
                        .expect(&format!("Encountered ] with unmatched [ at {}", code_index))
                }
            },
            _ => (),
        }
        code_index += 1;
    }
}

fn read_char() -> u8 {
    enable_raw_mode().expect("There was an error enabling raw mode for reading input"); //allows capturing of key events
    let chr = loop {
        if let Event::Key(key_event) =
            event::read().expect("There was an error reading a key event")
        {
            if key_event.kind == KeyEventKind::Release {
                //ignores key up event
                continue;
            }
            if let KeyCode::Char(c) = key_event.code {
                break c;
            }
        };
    };
    disable_raw_mode().expect("There was an error disabling raw mode for reading input");
    chr as u8
}
