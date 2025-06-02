use std::collections::HashMap;

use crate::token_data::{ token_defs::*, TokenMethods };

pub mod writer;
pub mod reader;
pub mod parser;

pub fn get_supported_tokens() -> HashMap<String, fn() -> Box<dyn TokenMethods>> {
    let mut tokens: HashMap<String, fn() -> Box<dyn TokenMethods>> = HashMap::new();

    // Add to Beginning
    tokens.insert("atb".to_string(), || Box::new(atb::Atb::default()));
    // Add to end
    tokens.insert("ate".to_string(), || Box::new(ate::Ate::default()));
    // Delete After
    tokens.insert("dla".to_string(), || Box::new(dla::Dla::default()));
    // Delete before
    tokens.insert("dlb".to_string(), || Box::new(dlb::Dlb::default()));
    // Delete chunk
    tokens.insert("dlc".to_string(), || Box::new(dlc::Dlc::default()));
    // Delete first
    tokens.insert("dlf".to_string(), || Box::new(dlf::Dlf::default()));
    // Delete last
    tokens.insert("dll".to_string(), || Box::new(dll::Dll::default()));
    // Replace all with
    tokens.insert("raw".to_string(), || Box::new(raw::Raw::default()));
    // Replace first with
    tokens.insert("rfw".to_string(), || Box::new(rfw::Rfw::default()));
    // Repeat
    tokens.insert("rpt".to_string(), || Box::new(rpt::Rpt::default()));
    // Rotate right
    tokens.insert("rtr".to_string(), || Box::new(rtr::Rtr::default()));
    // Rotate left
    tokens.insert("rtl".to_string(), || Box::new(rtl::Rtl::default()));
    // Trim both sides
    tokens.insert("tbs".to_string(), || Box::new(tbs::Tbs::default()));
    // Trim left side
    tokens.insert("tls".to_string(), || Box::new(tls::Tls::default()));
    // Trim right side
    tokens.insert("trs".to_string(), || Box::new(trs::Trs::default()));

    tokens
}
