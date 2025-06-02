// Internal

mod utils;
mod token_data;

// Core

pub mod text_parser;
pub mod builder;

// Bytecode
#[cfg(feature = "bytecode")]
pub mod bytecode_parser;
