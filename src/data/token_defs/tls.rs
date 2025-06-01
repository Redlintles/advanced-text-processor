use crate::data::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
// Trim left side
#[derive(Clone, Copy)]
pub struct Tls {}

impl Default for Tls {
    fn default() -> Self {
        return Tls {};
    }
}

impl TokenMethods for Tls {
    fn token_to_atp_line(&self) -> String {
        "tls;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        String::from(input.trim_start())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "tls;"

        if line[0] == "tls" {
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }
    fn get_string_repr(&self) -> String {
        "tls".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tls {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Tls::new().get_opcode() {
            return Ok(());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Tls::new().get_opcode(),
            operands: [].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x06
    }
}
