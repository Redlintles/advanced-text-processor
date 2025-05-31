use regex::Regex;

use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods, TokenOpCodes },
    data::TokenMethods,
};
// Replace first with
#[derive(Clone)]
pub struct Rfw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Rfw {
    pub fn params(pattern: String, text_to_replace: String) -> Self {
        Rfw {
            pattern: Regex::new(&pattern).expect("Failed creating regex"),
            text_to_replace,
        }
    }
    pub fn new() -> Self {
        Rfw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
        }
    }
}

impl TokenMethods for Rfw {
    fn token_to_atp_line(&self) -> String {
        format!("rfw {} {};\n", self.pattern, self.text_to_replace)
    }

    fn parse(&self, input: &str) -> String {
        self.pattern.replace(input, &self.text_to_replace).to_string()
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "rfw;"

        if line[0] == "rfw" {
            self.pattern = Regex::new(&line[1]).expect("Failed Creating Regex");
            self.text_to_replace = line[2].clone();
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "rfw".to_string()
    }
}

impl BytecodeTokenMethods for Rfw {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == TokenOpCodes::ReplaceFirstWith {
            if !(instruction.operands[0].is_empty() || instruction.operands[1].is_empty()) {
                self.pattern = Regex::new(&instruction.operands[0].clone()).expect(
                    "Parse error, Could not create regex"
                );
                self.text_to_replace = instruction.operands[1].clone();
                return Ok(());
            }

            return Err("An ATP Bytecode parsing error ocurred: Invalid Operands".to_string());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: TokenOpCodes::ReplaceFirstWith,
            operands: [self.pattern.to_string(), self.text_to_replace.to_string()].to_vec(),
        }
    }
}
