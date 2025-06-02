// Internal

mod utils;

#[cfg(not(feature = "test_access"))]
mod token_data;

#[cfg(feature = "test_access")]
pub mod token_data;

// Core

pub mod text_parser;
pub mod builder;

// Bytecode
#[cfg(feature = "bytecode")]
pub mod bytecode_parser;
