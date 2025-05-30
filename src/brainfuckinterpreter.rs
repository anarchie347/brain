//based on https://github.com/anarchie347/rusty-bf-interpreter
//adapted for this specific use case with addition of debug symbols for development and testing of the compiler

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::Write;
use std::time::Duration;
use std::{env, fs, io, mem};

pub fn run(brainfuck: &String) {
    let source = brainfuck.as_bytes();
    let mut mem_tape = [0u8; 30000];
    execute(source, &mut mem_tape, 0, true).expect("It crashed");
}

fn execute(
    source: &[u8],
    mem_tape: &mut [u8],
    initial_pointer_pos: usize,
    timings: bool,
) -> Result<(), BFExecuteError> {
    let mut loop_index_stack: Vec<usize> = Vec::new();
    let mut code_index: usize = 0;

    let mut pointer: usize = initial_pointer_pos;

    let mut input_time = Duration::new(0, 0);
    let total_time = std::time::Instant::now();

    while code_index < source.len() {
        match source[code_index] {
            b'+' => mem_tape[pointer] = mem_tape[pointer].wrapping_add(1),
            b'-' => mem_tape[pointer] = mem_tape[pointer].wrapping_sub(1),
            b'>' => {
                pointer += 1;
                if pointer == mem_tape.len() {
                    return Err(BFExecuteError::PointerOverflow(mem_tape.len()));
                }
            }
            b'<' => {
                if pointer == 0 {
                    return Err(BFExecuteError::PointerUnderflow);
                }
                pointer -= 1;
            }
            b'.' => write_char(mem_tape[pointer]),
            b',' => {
                let start_input = std::time::Instant::now();
                mem_tape[pointer] = read_char();
                input_time += start_input.elapsed();
            }
            b'[' => match mem_tape[pointer] {
                0 => {
                    //move pointer to index of associated closing bracket
                    let mut open_bracket_counter = 0;
                    while open_bracket_counter >= 0 {
                        code_index += 1;
                        if code_index == source.len() {
                            return Err(BFExecuteError::MissingCloseBracket);
                        }
                        match source[code_index] {
                            b'[' => open_bracket_counter += 1,
                            b']' => open_bracket_counter -= 1,
                            _ => (),
                        }
                    }
                }
                _ => loop_index_stack.push(code_index),
            },
            b']' => match mem_tape[pointer] {
                0 => _ = loop_index_stack.pop(),
                _ => {
                    code_index = match loop_index_stack.last() {
                        Some(i) => *i,
                        None => return Err(BFExecuteError::MissingOpenBracket),
                    }
                }
            },
            b'?' => println!("CELL[{}] VAL: {}", pointer, mem_tape[pointer]), //for debugging purposes
            b'#' => {
                let val32bit = mem_tape[pointer] as u32
                    + 256 * mem_tape[pointer - 1] as u32
                    + 65536 * mem_tape[pointer - 2] as u32
                    + 16777216 * mem_tape[pointer - 3] as u32;
                println!(
                    "BLOCK[{}] VAL: {} W:{}",
                    (pointer - 4) / 5,
                    val32bit,
                    mem_tape[pointer - 4]
                )
            }
            b'@' => {
                println!(
                    "W0 {} | D3 {} | D2 {} | D1 {} | D0 {}",
                    mem_tape[pointer - 4],
                    mem_tape[pointer - 3],
                    mem_tape[pointer - 2],
                    mem_tape[pointer - 1],
                    mem_tape[pointer]
                )
            }
            _ => (),
        }
        code_index += 1;
    }
    if timings {
        let total_duration = total_time.elapsed();
        println!();
        println!("Total execution time: {}ms", total_duration.as_millis());
        println!(
            "Compute time: {}ms",
            (total_duration - input_time).as_millis()
        );
    };
    Ok(())
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
            check_key_event_quit(key_event);
            if let Some(ascii) = key_code_to_ascii(key_event.code) {
                break ascii;
            }
        };
    };
    disable_raw_mode().expect("There was an error disabling raw mode for reading input");
    write_char(chr);
    chr
}

fn write_char(chr: u8) {
    //used to correctly deal with newline characters. BF uses 10 as newline, but windows uses 10 and 13
    if chr == 10 {
        println!();
    } else {
        print!("{}", chr as char);
        io::stdout().flush().expect("Error printing character");
    }
}

fn check_key_event_quit(key_event: KeyEvent) {
    //allows Ctrl+C to still be used to quit process when reading a key input
    if key_event.modifiers.contains(KeyModifiers::CONTROL)
        && (key_event.code == KeyCode::Char('c') || key_event.code == KeyCode::Char('C'))
    {
        std::process::exit(0);
    }
}

fn key_code_to_ascii(key_code: KeyCode) -> Option<u8> {
    //maps some non-display characters not matched by KeyCode::Char to corresponding ascii values
    match key_code {
        KeyCode::Char(c) => Some(c as u8),
        KeyCode::Backspace => Some(8),
        KeyCode::Enter => Some(10),
        KeyCode::Tab => Some(9),
        KeyCode::BackTab => Some(9),
        KeyCode::Esc => Some(27),
        _ => None,
    }
}

#[derive(Debug)]
enum BFExecuteError {
    PointerUnderflow,
    PointerOverflow(usize),
    MissingCloseBracket,
    MissingOpenBracket,
}
