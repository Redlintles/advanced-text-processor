use crate::{ bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods }, data::TokenMethods };
// Delete first
#[derive(Clone, Copy)]
pub struct Dlf {}

impl Dlf {
    pub fn new() -> Self {
        Dlf {}
    }
}

impl TokenMethods for Dlf {
    fn token_to_atp_line(&self) -> String {
        "dlf;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);
        s.drain(..1);
        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "dlf;"

        if line[0] == "dlf" {
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "dlf".to_string()
    }
}

impl BytecodeTokenMethods for Dlf {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Dlf::new().get_opcode() {
            return Ok(());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Dlf::new().get_opcode(),
            operands: [].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x03
    }
}
