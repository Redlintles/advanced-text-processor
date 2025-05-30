use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods, TokenOpCodes },
    data::TokenMethods,
};
#[derive(Clone)]
pub struct Rpt {
    pub times: usize,
}

impl Rpt {
    pub fn params(times: usize) -> Self {
        Rpt { times }
    }

    pub fn new() -> Self {
        return Rpt {
            times: 0,
        };
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

impl BytecodeTokenMethods for Rpt {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == TokenOpCodes::Repeat {
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
            op_code: TokenOpCodes::Repeat,
            operands: [self.times.to_string()].to_vec(),
        }
    }
}
