use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods, TokenOpCodes },
    data::TokenMethods,
};

// Trim both sides
#[derive(Clone)]
pub struct Tbs {}

impl Tbs {
    pub fn new() -> Self {
        Tbs {}
    }
}

impl TokenMethods for Tbs {
    fn token_to_atp_line(&self) -> String {
        "tbs;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        String::from(input.trim())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "tbs;"

        if line[0] == "tbs" {
            return Ok(());
        }
        Err("Erro de parsing".to_string())
    }

    fn get_string_repr(&self) -> String {
        "tbs".to_string()
    }
}

impl BytecodeTokenMethods for Tbs {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == TokenOpCodes::TrimBothSides {
            return Ok(());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: TokenOpCodes::TrimBothSides,
            operands: [].to_vec(),
        }
    }
}
