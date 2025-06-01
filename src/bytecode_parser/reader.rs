use std::{ fs::OpenOptions, io::{ BufRead, BufReader }, path::Path };

use super::{
    get_supported_bytecode_tokens,
    token_from_hex_string,
    BytecodeInstruction,
    BytecodeTokenMethods,
};

pub fn read_bytecode_from_file(path: &Path) -> Result<Vec<Box<dyn BytecodeTokenMethods>>, String> {
    let mut result: Vec<Box<dyn BytecodeTokenMethods>> = Vec::new();

    let file = match OpenOptions::new().read(true).open(path) {
        Ok(x) => x,
        Err(_) => {
            return Err("An Atp Bytecode reading error ocurred: Could not open file".to_string());
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_text = match line {
            Ok(x) => x,
            Err(_) => {
                return Err("An ATP Parsing error ocurred: Error reading file line".to_string());
            }
        };

        let chunks = match shell_words::split(&line_text) {
            Ok(x) => x,
            Err(_) => {
                return Err("An ATP Parsing error ocurred: Error splitting file line".to_string());
            }
        };

        let brute_op_code = match chunks.first() {
            Some(x) => x,
            None => {
                return Err("An ATP Parsing error ocurred: Parsing error".to_string());
            }
        };

        let parsed_op_code = match token_from_hex_string(&brute_op_code) {
            Some(x) => x,
            None => {
                return Err("An ATP Parsing error ocurred: Parsing error".to_string());
            }
        };

        let instruction = BytecodeInstruction {
            op_code: parsed_op_code,
            operands: chunks,
        };

        let op_code_map = get_supported_bytecode_tokens();

        let mapped_token = match op_code_map.get(&(parsed_op_code as u8)) {
            Some(x) => x,
            None => {
                return Err("An ATP Parsing error ocurred: Token not found".to_string());
            }
        };

        let mut new_token = mapped_token();

        match new_token.token_from_bytecode_instruction(instruction) {
            Ok(()) => result.push(new_token),
            Err(_) => {
                return Err("An ATP Parsing error ocurred: Token not found".to_string());
            }
        }
    }

    Ok(result)
}
