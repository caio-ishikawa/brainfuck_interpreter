use std::env;
use std::fs;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
enum Operations { 
    IncrementPtr,
    DecrementPtr,
    IncrementByte,
    DecrementByte,
    Read,
    Write,
    StartLoop,
    EndLoop
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let file_name = &cli_args[1];
    let code = lexer(file_name);

    let parsed_code = parse(code);
    compile(parsed_code);
}

// Reads file and returns list of chars containing the brainfuck commands //
fn lexer(file_name: &String) -> Vec<char> {
    let input = fs::read_to_string(file_name).expect("Error reading file");
    let code_vector: Vec<char> = input.chars().filter(|&n| n == '>' || n == '<' || n == '+' || n == '-' || n == '[' || n == ']' || n == '.' || n == ',').collect();
    return code_vector;
}

// Turns list of brainfuck commands into a vector of type Operations //
fn parse(brain_code: Vec<char>) -> Vec<Operations> { 
    let mut code: Vec<Operations> = Vec::new();

    for i in brain_code { 
        let operation = match i { 
            '+' => Operations::IncrementByte,
            '-' => Operations::DecrementByte,
            '>' => Operations::IncrementPtr,
            '<' => Operations::DecrementPtr,
            '.' => Operations::Read,
            ',' => Operations::Write, 
            '[' => Operations::StartLoop,
            ']' => Operations::EndLoop,
            _ => Operations::EndLoop,
        };
        code.push(operation);
    }
    return code;
}

// Runs code and prints the memory tape for debugging purposes || WILL CHANGE LATER //
fn compile(code: Vec<Operations>) {
    let mut memory: Vec<u8> = vec![0; 1000];
    let mut mem_ptr = 0;
    let mut code_ptr = 0;
    let mut bracket_idx: Vec<usize> = Vec::new();
    let mut output: Vec<u8> = Vec::new();

    while code_ptr < code.len() { 
        let command = code[code_ptr]; 

        match command { 
            Operations::IncrementByte => memory[mem_ptr] = memory[mem_ptr].wrapping_add(1),
            Operations::DecrementByte => memory[mem_ptr] = memory[mem_ptr].wrapping_sub(1),
            Operations::IncrementPtr => mem_ptr += 1,
            Operations::DecrementPtr => mem_ptr -= 1, 
            Operations::Read => output.push(memory[mem_ptr]),   
            Operations::StartLoop => bracket_idx.push(code_ptr), 
            Operations::EndLoop => { 
                if memory[mem_ptr] != 0 {
                    code_ptr = *bracket_idx.last().unwrap()
                }
                else {
                    bracket_idx.pop();
                }
            }, 
            _ => println!("ERROR") 
        };
        code_ptr += 1;
    }
    //println!("{:?}", memory);
    log_ptr(output);
}

fn log_ptr(byte: Vec<u8>) { 
    let int_as_char: Vec<char> = byte.iter().map(|&n| n as char).collect();
    let output_as_str: String = int_as_char.into_iter().collect();
    println!("OUTPUT: {:?}", output_as_str);
}
