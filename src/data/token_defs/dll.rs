use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods, TokenOpCodes },
    data::TokenMethods,
};

// Delete last
#[derive(Clone, Copy)]
pub struct Dll {}

impl Dll {
    pub fn new() -> Self {
        Dll {}
    }
}

impl TokenMethods for Dll {
    fn token_to_atp_line(&self) -> String {
        "dll;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);

        if let Some((x, _)) = s.char_indices().rev().next() {
            s.drain(x..);
        }

        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "dll;"

        if line[0] == "dll" {
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "dll".to_string()
    }
}

impl BytecodeTokenMethods for Dll {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == TokenOpCodes::DeleteLast {
            return Ok(());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: TokenOpCodes::DeleteLast,
            operands: [].to_vec(),
        }
    }
}
