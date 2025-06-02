use crate::token_data::{ TokenMethods };

use crate::token_data::token_defs::*;

use super::atp_processor::{ AtpProcessorMethods, AtpProcessor };
#[cfg(feature = "debug")]
use super::atp_processor::AtpProcessorDebugMethods;
#[cfg(feature = "bytecode")]
use super::atp_processor::AtpProcessorBytecodeMethods;
#[cfg(feature = "bytecode_debug")]
use super::atp_processor::AtpProcessorBytecodeDebugMethods;
#[derive(Default)]
pub struct AtpBuilder {
    tokens: Vec<Box<dyn TokenMethods>>,
}
#[derive(Default)]
pub struct AtpProcessorBuilder {
    tokens: Vec<Box<dyn TokenMethods>>,
}

impl AtpProcessorBuilder {
    pub fn text_processor(self) -> (Box<dyn AtpProcessorMethods>, String) {
        let mut processor: Box<dyn AtpProcessorMethods> = Box::new(AtpProcessor::new());
        let identifier = processor.add_transform(self.tokens);
        (processor, identifier)
    }
    #[cfg(feature = "debug")]
    pub fn text_debug_processor(self) -> (Box<dyn AtpProcessorDebugMethods>, String) {
        let mut processor: Box<dyn AtpProcessorDebugMethods> = Box::new(AtpProcessor::new());
        let identifier = processor.add_transform(self.tokens);
        (processor, identifier)
    }
    #[cfg(feature = "bytecode")]
    pub fn bytecode_processor(self) -> (Box<dyn AtpProcessorBytecodeMethods>, String) {
        let mut processor: Box<dyn AtpProcessorBytecodeMethods> = Box::new(AtpProcessor::new());
        let identifier = processor.add_transform(self.tokens);
        (processor, identifier)
    }
    #[cfg(feature = "bytecode_debug")]
    pub fn bytecode_debug_processor(self) -> (Box<dyn AtpProcessorBytecodeDebugMethods>, String) {
        let mut processor: Box<dyn AtpProcessorBytecodeDebugMethods> = Box::new(
            AtpProcessor::new()
        );
        let identifier = processor.add_transform(self.tokens);
        (processor, identifier)
    }
}

impl AtpBuilder {
    pub fn new() -> AtpBuilder {
        AtpBuilder {
            tokens: Vec::new(),
        }
    }

    pub fn build(self) -> AtpProcessorBuilder {
        AtpProcessorBuilder {
            tokens: self.tokens,
        }
    }
}

impl AtpBuilder {
    pub fn trim_both(mut self) -> Self {
        self.tokens.push(Box::new(tbs::Tbs::default()));
        self
    }

    pub fn trim_left(mut self) -> Self {
        self.tokens.push(Box::new(tls::Tls::default()));
        self
    }
    pub fn trim_right(mut self) -> Self {
        self.tokens.push(Box::new(trs::Trs::default()));
        self
    }
    pub fn add_to_end(mut self, text: &str) -> Self {
        self.tokens.push(Box::new(ate::Ate::params(text.to_string())));
        self
    }
    pub fn add_to_beginning(mut self, text: &str) -> Self {
        self.tokens.push(Box::new(atb::Atb::params(text.to_string())));
        self
    }
    pub fn delete_first(mut self) -> Self {
        self.tokens.push(Box::new(dlf::Dlf::default()));
        self
    }
    pub fn delete_last(mut self) -> Self {
        self.tokens.push(Box::new(dll::Dll::default()));
        self
    }
    pub fn delete_after(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(dla::Dla::params(index)));
        self
    }
    pub fn delete_before(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(dlb::Dlb::params(index)));
        self
    }
    pub fn delete_chunk(mut self, start_index: usize, end_index: usize) -> Self {
        self.tokens.push(Box::new(dlc::Dlc::params(start_index, end_index)));
        self
    }
    pub fn replace_all_with(mut self, pattern: &str, text_to_replace: &str) -> Self {
        self.tokens.push(
            Box::new(match raw::Raw::params(pattern.to_string(), text_to_replace.to_string()) {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            })
        );

        self
    }
    pub fn replace_first_with(mut self, pattern: &str, text_to_replace: &str) -> Self {
        self.tokens.push(
            Box::new(match rfw::Rfw::params(pattern.to_string(), text_to_replace.to_string()) {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            })
        );
        self
    }
    pub fn rotate_left(mut self, times: usize) -> Self {
        self.tokens.push(Box::new(rtl::Rtl::params(times)));
        self
    }
    pub fn rotate_right(mut self, times: usize) -> Self {
        self.tokens.push(Box::new(rtr::Rtr::params(times)));
        self
    }
    pub fn repeat(mut self, times: usize) -> Self {
        self.tokens.push(Box::new(rpt::Rpt::params(times)));
        self
    }
}
