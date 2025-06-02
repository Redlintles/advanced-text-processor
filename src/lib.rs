// Internal

mod utils;

#[cfg(not(feature = "test_access"))]
mod token_data;

#[cfg(feature = "test_access")]
pub mod token_data;

#[cfg(not(feature = "test_access"))]
mod text_parser;

#[cfg(feature = "test_access")]
pub mod text_parser;
// Public

pub mod builder;

// Bytecode
#[cfg(feature = "bytecode")]
pub mod bytecode_parser;
