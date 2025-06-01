use crate::data::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

// add to beginning
#[derive(Clone, Default)]
pub struct Atb {
    pub text: String,
}

impl Atb {
    pub fn params(text: String) -> Self {
        Atb {
            text,
        }
    }
}

impl TokenMethods for Atb {
    fn token_to_atp_line(&self) -> String {
        format!("atb {};\n", self.text)
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(&self.text);
        s.push_str(input);
        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "atb;"

        if line[0] == "atb" {
            self.text = line[1].clone();
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "atb".to_string()
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Atb {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Atb::default().get_opcode() {
            if !instruction.operands[0].is_empty() {
                self.text = instruction.operands[1].clone();
                return Ok(());
            }

            return Err("An ATP Bytecode parsing error ocurred: Invalid Operands".to_string());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Atb::default().get_opcode(),
            operands: [self.text.clone()].to_vec(),
        }
    }

    fn get_opcode(&self) -> u8 {
        0x01
    }
}
