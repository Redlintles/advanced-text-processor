use crate::token_data::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
// add to end
#[derive(Clone, Default)]
pub struct Ate {
    pub text: String,
}

impl Ate {
    pub fn params(text: String) -> Self {
        Ate {
            text,
        }
    }
}

impl TokenMethods for Ate {
    fn token_to_atp_line(&self) -> String {
        format!("ate {};\n", self.text)
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);
        s.push_str(&self.text);
        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "ate;"

        if line[0] == "ate" {
            self.text = line[1].clone();
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "ate".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ate {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Ate::default().get_opcode() {
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
            op_code: Ate::default().get_opcode(),
            operands: [self.text.clone()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x02
    }
}
