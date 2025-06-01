use crate::data::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

// Trim right side
#[derive(Clone, Copy)]
pub struct Trs {}

impl Default for Trs {
    fn default() -> Self {
        return Trs {};
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
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Trs {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Trs::new().get_opcode() {
            return Ok(());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Trs::new().get_opcode(),
            operands: [].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x07
    }
}
