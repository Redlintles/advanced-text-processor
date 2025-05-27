use regex::Regex;

use crate::data::{ AtpToken, TokenMethods };

use crate::data::token_defs::*;

use super::atp_processor::{ AtpProcessor, AtpProcessorMethods };

pub struct AtpBuilder {
    tokens: Vec<AtpToken>,
}
impl AtpBuilder {
    pub fn new() -> AtpBuilder {
        AtpBuilder {
            tokens: Vec::new(),
        }
    }

    pub fn build(&self) -> (AtpProcessor, String) {
        let mut processor = AtpProcessor::new();

        let identifier = processor.add_transform(&self.tokens);

        (processor, identifier)
    }
}

impl AtpBuilder {
    pub fn trim_both(mut self) -> Self {
        self.tokens.push(AtpToken::Tbs(tbs::Tbs::new()));
        self
    }

    pub fn trim_left(mut self) -> Self {
        self.tokens.push(AtpToken::Tls(tls::Tls {}));
        self
    }
    pub fn trim_right(mut self) -> Self {
        self.tokens.push(AtpToken::Trs(trs::Trs {}));
        self
    }
    pub fn add_to_end(mut self, text: &str) -> Self {
        self.tokens.push(AtpToken::Ate(ate::Ate { text: text.to_string() }));
        self
    }
    pub fn add_to_beginning(mut self, text: &str) -> Self {
        self.tokens.push(AtpToken::Atb(atb::Atb { text: text.to_string() }));
        self
    }
    pub fn delete_first(mut self) -> Self {
        self.tokens.push(AtpToken::Dlf(dlf::Dlf {}));
        self
    }
    pub fn delete_last(mut self) -> Self {
        self.tokens.push(AtpToken::Dll(dll::Dll {}));
        self
    }
    pub fn delete_after(mut self, index: usize) -> Self {
        self.tokens.push(AtpToken::Dla(dla::Dla { index }));
        self
    }
    pub fn delete_before(mut self, index: usize) -> Self {
        self.tokens.push(AtpToken::Dlb(dlb::Dlb { index }));
        self
    }
    pub fn delete_chunk(mut self, start_index: usize, end_index: usize) -> Self {
        self.tokens.push(AtpToken::Dlc(dlc::Dlc { start_index, end_index }));
        self
    }
    pub fn replace_all_with(mut self, pattern: &str, text_to_replace: &str) -> Self {
        self.tokens.push(
            AtpToken::Raw(raw::Raw {
                pattern: Regex::new(pattern).expect("Failed creating regex"),
                text_to_replace: text_to_replace.to_string(),
            })
        );
        self
    }
    pub fn replace_first_with(mut self, pattern: &str, text_to_replace: &str) -> Self {
        self.tokens.push(
            AtpToken::Rfw(rfw::Rfw {
                pattern: Regex::new(pattern).expect("Failed creating regex"),
                text_to_replace: text_to_replace.to_string(),
            })
        );
        self
    }
    pub fn rotate_left(mut self, times: usize) -> Self {
        self.tokens.push(AtpToken::Rtl(rtl::Rtl { times }));
        self
    }
    pub fn rotate_right(mut self, times: usize) -> Self {
        self.tokens.push(AtpToken::Rtr(rtr::Rtr { times }));
        self
    }
    pub fn repeat(mut self, times: usize) -> Self {
        self.tokens.push(AtpToken::Rpt(rpt::Rpt { times }));
        self
    }
}
