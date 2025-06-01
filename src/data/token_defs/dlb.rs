use crate::data::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
// Delete before
#[derive(Clone, Copy, Default)]
pub struct Dlb {
    pub index: usize,
}

impl Dlb {
    pub fn params(index: usize) -> Self {
        Dlb {
            index,
        }
    }
}

impl TokenMethods for Dlb {
    fn token_to_atp_line(&self) -> String {
        format!("dlb {};\n", self.index)
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);

        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index)
                .map(|(i, _)| i)
        {
            s.drain(..=byte_index);
        }

        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "dlb;"

        if line[0] == "dlb" {
            self.index = line[1].clone().parse().expect("Parse from string to usize failed");
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "dlb".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dlb {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Dlb::default().get_opcode() {
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
            op_code: Dlb::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0a
    }
}
