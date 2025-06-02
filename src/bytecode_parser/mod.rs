use crate::{ token_data::{ TokenMethods }, utils::mapping::get_supported_bytecode_tokens };

pub mod writer;
pub mod reader;
pub mod transformer;

pub struct BytecodeInstruction {
    pub op_code: u8,
    pub operands: Vec<String>,
}

impl BytecodeInstruction {
    pub fn to_bytecode_line(&self) -> String {
        let mut result = format!("{:#04x}", self.op_code as u8);
        for operand in self.operands.iter() {
            result = format!("{} {}", result, operand);
        }

        result.push_str("\n");

        result
    }
}

pub trait BytecodeTokenMethods: TokenMethods {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String>;
    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction;

    fn get_opcode(&self) -> u8;
}

pub fn token_from_hex_string(s: &str) -> Option<u8> {
    let stripped = s.strip_prefix("0x")?;

    let byte = u8::from_str_radix(stripped, 16).ok()?;

    Some(byte)
}

pub fn token_to_bytecode_token_convert(
    token: Box<dyn TokenMethods>
) -> Result<Box<dyn BytecodeTokenMethods>, String> {
    let mut line = token.token_to_atp_line();

    line.pop();

    let chunks = match shell_words::split(&line) {
        Ok(x) => x,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let string_token_map = get_supported_bytecode_tokens();

    let factory = match string_token_map.get(&token.get_string_repr()) {
        Some(x) => x,
        None => {
            return Err("Invalid Token".to_string());
        }
    };

    let mut new_token = factory();

    new_token.token_from_vec_params(chunks)?;

    Ok(new_token)
}
