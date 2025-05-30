use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods, TokenOpCodes },
    data::TokenMethods,
};

// Trim right side
#[derive(Clone, Copy)]
pub struct Trs {}

impl Trs {
    pub fn new() -> Self {
        Trs {}
    }
}

impl TokenMethods for Trs {
    fn token_to_atp_line(&self) -> String {
        "trs;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        String::from(input.trim_end())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "trs;"

        if line[0] == "trs" {
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "trs".to_string()
    }
}

impl BytecodeTokenMethods for Trs {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == TokenOpCodes::TrimRightSide {
            return Ok(());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: TokenOpCodes::TrimRightSide,
            operands: [].to_vec(),
        }
    }
}
