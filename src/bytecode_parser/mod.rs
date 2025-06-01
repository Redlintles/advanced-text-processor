use std::collections::HashMap;

use crate::data::{
    token_defs::{
        atb::Atb,
        ate::Ate,
        dla::Dla,
        dlb::Dlb,
        dlc::Dlc,
        dlf::Dlf,
        dll::Dll,
        raw::Raw,
        rfw::Rfw,
        rpt::Rpt,
        rtl::Rtl,
        rtr::Rtr,
        tbs::Tbs,
        tls::Tls,
        trs::Trs,
    },
    TokenMethods,
};

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

    let string_token_map = get_map_str_to_bytecode_token();

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

pub fn get_map_bytecode_to_bytecode_token() -> HashMap<u8, fn() -> Box<dyn BytecodeTokenMethods>> {
    let mut bytecode_to_token: HashMap<u8, fn() -> Box<dyn BytecodeTokenMethods>> = HashMap::new();

    bytecode_to_token.insert(0x01, || Box::new(Atb::new()));
    bytecode_to_token.insert(0x02, || Box::new(Ate::new()));
    bytecode_to_token.insert(0x03, || Box::new(Dlf::new()));
    bytecode_to_token.insert(0x04, || Box::new(Dll::new()));
    bytecode_to_token.insert(0x05, || Box::new(Tbs::new()));
    bytecode_to_token.insert(0x06, || Box::new(Tls::new()));
    bytecode_to_token.insert(0x07, || Box::new(Trs::new()));
    bytecode_to_token.insert(0x08, || Box::new(Dlc::new()));
    bytecode_to_token.insert(0x09, || Box::new(Dla::new()));
    bytecode_to_token.insert(0x0a, || Box::new(Dlb::new()));
    bytecode_to_token.insert(0x0b, || Box::new(Raw::new()));
    bytecode_to_token.insert(0x0c, || Box::new(Rfw::new()));
    bytecode_to_token.insert(0x0d, || Box::new(Rpt::new()));
    bytecode_to_token.insert(0x0e, || Box::new(Rtl::new()));
    bytecode_to_token.insert(0x0f, || Box::new(Rtr::new()));

    bytecode_to_token
}

pub fn get_map_str_to_bytecode_token() -> HashMap<String, fn() -> Box<dyn BytecodeTokenMethods>> {
    let mut string_to_token: HashMap<
        String,
        fn() -> Box<dyn BytecodeTokenMethods>
    > = HashMap::new();
    string_to_token.insert("atb".to_string(), || Box::new(Atb::new()));
    string_to_token.insert("ate".to_string(), || Box::new(Ate::new()));
    string_to_token.insert("dlf".to_string(), || Box::new(Dlf::new()));
    string_to_token.insert("dll".to_string(), || Box::new(Dll::new()));
    string_to_token.insert("tbs".to_string(), || Box::new(Tbs::new()));
    string_to_token.insert("tls".to_string(), || Box::new(Tls::new()));
    string_to_token.insert("trs".to_string(), || Box::new(Trs::new()));
    string_to_token.insert("dlc".to_string(), || Box::new(Dlc::new()));
    string_to_token.insert("dla".to_string(), || Box::new(Dla::new()));
    string_to_token.insert("dlb".to_string(), || Box::new(Dlb::new()));
    string_to_token.insert("raw".to_string(), || Box::new(Raw::new()));
    string_to_token.insert("rfw".to_string(), || Box::new(Rfw::new()));
    string_to_token.insert("rpt".to_string(), || Box::new(Rpt::new()));
    string_to_token.insert("rtl".to_string(), || Box::new(Rtl::new()));
    string_to_token.insert("rtr".to_string(), || Box::new(Rtr::new()));

    string_to_token
}
