use std::collections::HashMap;

use crate::token_data::{
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

    bytecode_to_token.insert(0x01, || Box::new(Atb::default()));
    bytecode_to_token.insert(0x02, || Box::new(Ate::default()));
    bytecode_to_token.insert(0x03, || Box::new(Dlf::default()));
    bytecode_to_token.insert(0x04, || Box::new(Dll::default()));
    bytecode_to_token.insert(0x05, || Box::new(Tbs::default()));
    bytecode_to_token.insert(0x06, || Box::new(Tls::default()));
    bytecode_to_token.insert(0x07, || Box::new(Trs::default()));
    bytecode_to_token.insert(0x08, || Box::new(Dlc::default()));
    bytecode_to_token.insert(0x09, || Box::new(Dla::default()));
    bytecode_to_token.insert(0x0a, || Box::new(Dlb::default()));
    bytecode_to_token.insert(0x0b, || Box::new(Raw::default()));
    bytecode_to_token.insert(0x0c, || Box::new(Rfw::default()));
    bytecode_to_token.insert(0x0d, || Box::new(Rpt::default()));
    bytecode_to_token.insert(0x0e, || Box::new(Rtl::default()));
    bytecode_to_token.insert(0x0f, || Box::new(Rtr::default()));

    bytecode_to_token
}

pub fn get_map_str_to_bytecode_token() -> HashMap<String, fn() -> Box<dyn BytecodeTokenMethods>> {
    let mut string_to_token: HashMap<
        String,
        fn() -> Box<dyn BytecodeTokenMethods>
    > = HashMap::new();
    string_to_token.insert("atb".to_string(), || Box::new(Atb::default()));
    string_to_token.insert("ate".to_string(), || Box::new(Ate::default()));
    string_to_token.insert("dlf".to_string(), || Box::new(Dlf::default()));
    string_to_token.insert("dll".to_string(), || Box::new(Dll::default()));
    string_to_token.insert("tbs".to_string(), || Box::new(Tbs::default()));
    string_to_token.insert("tls".to_string(), || Box::new(Tls::default()));
    string_to_token.insert("trs".to_string(), || Box::new(Trs::default()));
    string_to_token.insert("dlc".to_string(), || Box::new(Dlc::default()));
    string_to_token.insert("dla".to_string(), || Box::new(Dla::default()));
    string_to_token.insert("dlb".to_string(), || Box::new(Dlb::default()));
    string_to_token.insert("raw".to_string(), || Box::new(Raw::default()));
    string_to_token.insert("rfw".to_string(), || Box::new(Rfw::default()));
    string_to_token.insert("rpt".to_string(), || Box::new(Rpt::default()));
    string_to_token.insert("rtl".to_string(), || Box::new(Rtl::default()));
    string_to_token.insert("rtr".to_string(), || Box::new(Rtr::default()));

    string_to_token
}
