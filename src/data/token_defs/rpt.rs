use crate::data::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
#[derive(Clone, Default)]
pub struct Rpt {
    pub times: usize,
}

impl Rpt {
    pub fn params(times: usize) -> Self {
        Rpt { times }
    }
}

impl TokenMethods for Rpt {
    fn token_to_atp_line(&self) -> String {
        format!("rpt {};\n", self.times)
    }

    fn parse(&self, input: &str) -> String {
        input.repeat(self.times)
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        if line[0] == "rpt" {
            self.times = line[1].parse().expect("Parsing from string to usize failed");
            return Ok(());
        }

        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "rpt".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rpt {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Rpt::default().get_opcode() {
            if !(instruction.operands[0].is_empty() || instruction.operands[1].is_empty()) {
                self.times = instruction.operands[1]
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
            op_code: Rpt::default().get_opcode(),
            operands: [self.times.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0d
    }
}
