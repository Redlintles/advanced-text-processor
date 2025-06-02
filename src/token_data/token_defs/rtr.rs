use crate::token_data::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
#[derive(Clone, Default)]
pub struct Rtr {
    pub times: usize,
}

impl Rtr {
    pub fn params(times: usize) -> Rtr {
        Rtr {
            times,
        }
    }
}

impl TokenMethods for Rtr {
    fn parse(&self, input: &str) -> String {
        if input.is_empty() {
            return String::default();
        }

        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let times = self.times % len;

        chars[len - times..]
            .iter()
            .chain(&chars[..len - times])
            .collect()
    }

    fn token_to_atp_line(&self) -> String {
        format!("rtr {};\n", self.times)
    }
    fn get_string_repr(&self) -> String {
        "rtr".to_string()
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        if line[0] == "rtr" {
            self.times = line[1].parse().expect("Parsing from string to usize failed");
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rtr {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Rtr::default().get_opcode() {
            if !(instruction.operands[0].is_empty() || instruction.operands[1].is_empty()) {
                self.times = instruction.operands[0]
                    .clone()
                    .parse()
                    .expect("Parse error: Failed parsing string to usize");
                return Ok(());
            }

            return Err("An ATP Bytecode parsing error ocurred: Invalid Operands".to_string());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Rtr::default().get_opcode(),
            operands: [self.times.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0f
    }
}
