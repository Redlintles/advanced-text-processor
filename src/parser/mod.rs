use std::collections::HashMap;

use crate::data::{ token_defs::*, TokenMethods };

pub mod writer;
pub mod reader;
pub mod parser;

pub fn get_supported_tokens() -> HashMap<String, fn() -> Box<dyn TokenMethods>> {
    let mut tokens: HashMap<String, fn() -> Box<dyn TokenMethods>> = HashMap::new();

    // Add to Beginning
    tokens.insert("atb".to_string(), || Box::new(atb::Atb::new()));
    // Add to end
    tokens.insert("ate".to_string(), || Box::new(ate::Ate::new()));
    // Delete After
    tokens.insert("dla".to_string(), || Box::new(dla::Dla::new()));
    // Delete before
    tokens.insert("dlb".to_string(), || Box::new(dlb::Dlb::new()));
    // Delete chunk
    tokens.insert("dlc".to_string(), || Box::new(dlc::Dlc::new()));
    // Delete first
    tokens.insert("dlf".to_string(), || Box::new(dlf::Dlf::new()));
    // Delete last
    tokens.insert("dll".to_string(), || Box::new(dll::Dll::new()));
    // Replace all with
    tokens.insert("raw".to_string(), || Box::new(raw::Raw::new()));
    // Replace first with
    tokens.insert("rfw".to_string(), || Box::new(rfw::Rfw::new()));
    // Repeat
    tokens.insert("rpt".to_string(), || Box::new(rpt::Rpt::new()));
    // Rotate right
    tokens.insert("rtr".to_string(), || Box::new(rtr::Rtr::new()));
    // Rotate left
    tokens.insert("rtl".to_string(), || Box::new(rtl::Rtl::new()));
    // Trim both sides
    tokens.insert("tbs".to_string(), || Box::new(tbs::Tbs::new()));
    // Trim left side
    tokens.insert("tls".to_string(), || Box::new(tls::Tls::new()));
    // Trim right side
    tokens.insert("trs".to_string(), || Box::new(trs::Trs::new()));

    tokens
}
