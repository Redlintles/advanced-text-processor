use crate::data::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
// Delete after
#[derive(Clone, Copy)]
pub struct Dla {
    pub index: usize,
}

impl Dla {
    pub fn params(index: usize) -> Self {
        Dla {
            index,
        }
    }
    pub fn new() -> Self {
        Dla { index: 0 }
    }
}

impl TokenMethods for Dla {
    fn token_to_atp_line(&self) -> String {
        format!("dla {};\n", self.index)
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);

        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index)
                .map(|(i, _)| i)
        {
            s.drain(byte_index..);
        }

        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "dla;"

        if line[0] == "dla" {
            self.index = line[1].clone().parse().expect("Parse from string to usize failed");
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "dla".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dla {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Dla::new().get_opcode() {
            if !instruction.operands[0].is_empty() {
                self.index = instruction.operands[0]
                    .clone()
                    .parse()
                    .expect("Parse from string to usize failed");
                return Ok(());
            }

            return Err("An ATP Bytecode parsing error ocurred: Invalid Operands".to_string());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Dla::new().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }

    fn get_opcode(&self) -> u8 {
        0x09
    }
}
