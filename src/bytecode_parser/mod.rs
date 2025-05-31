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

pub fn get_supported_bytecode_tokens() -> HashMap<u8, fn() -> Box<dyn BytecodeTokenMethods>> {
    let mut result: HashMap<u8, fn() -> Box<dyn BytecodeTokenMethods>> = HashMap::new();

    result.insert(0x01, || Box::new(Atb::new()));
    result.insert(0x02, || Box::new(Ate::new()));
    result.insert(0x03, || Box::new(Dlf::new()));
    result.insert(0x04, || Box::new(Dll::new()));
    result.insert(0x05, || Box::new(Tbs::new()));
    result.insert(0x06, || Box::new(Tls::new()));
    result.insert(0x07, || Box::new(Trs::new()));
    result.insert(0x08, || Box::new(Dlc::new()));
    result.insert(0x09, || Box::new(Dla::new()));
    result.insert(0x0a, || Box::new(Dlb::new()));
    result.insert(0x0b, || Box::new(Raw::new()));
    result.insert(0x0c, || Box::new(Rfw::new()));
    result.insert(0x0d, || Box::new(Rpt::new()));
    result.insert(0x0e, || Box::new(Rtl::new()));
    result.insert(0x0f, || Box::new(Rtr::new()));

    result
}
