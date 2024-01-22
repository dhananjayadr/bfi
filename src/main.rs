use std::env;
use std::fs::File;
use std::io::Read;

fn brainfuck_interpreter(code: &str) -> String {
    let mut memory: [u8; 30000] = [0u8; 30000];
    let mut mem_ptr: usize = 0;
    let mut instr_ptr: usize = 0;
    let mut output: String = String::new();
    let mut loop_stack: Vec<usize> = Vec::new();

    while let Some(command) = code.chars().nth(instr_ptr) {
        match command {
            '>' => mem_ptr += 1,
            '<' => mem_ptr = mem_ptr.saturating_sub(1),
            '+' => memory[mem_ptr] = memory[mem_ptr].wrapping_add(1),
            '-' => memory[mem_ptr] = memory[mem_ptr].wrapping_sub(1),
            '[' => {
                if memory[mem_ptr] == 0 {
                    let mut depth: i32 = 1;
                    instr_ptr += 1;
                    while depth > 0 {
                        match code.chars().nth(instr_ptr).unwrap() {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                        instr_ptr += 1;
                    }
                } else {
                    loop_stack.push(instr_ptr);
                }
            }
            ']' => {
                if memory[mem_ptr] != 0 {
                    if let Some(start) = loop_stack.pop() {
                        instr_ptr = start;
                        continue; // Skip the increment at the end of the loop
                    }
                }
            }
            '.' => output.push(char::from(memory[mem_ptr])),
            ',' => {
                // Input is not implemented in this example
            }
            _ => {}
        }

        instr_ptr += 1;
    }

    output
}

fn main() {
    // Retrieve the first command line argument as the file path
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path: &String = &args[1];

    // Read Brainfuck code from the specified file
    let mut file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Unable to open the file.");
            return;
        }
    };

    let mut code: String = String::new();
    if let Err(_) = file.read_to_string(&mut code) {
        eprintln!("Error: Unable to read the file.");
        return;
    }

    // Call the interpreter function with the code read from the file
    let result = brainfuck_interpreter(&code);
    println!("{}", result);
}
