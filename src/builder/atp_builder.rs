use crate::data::{ AtpToken, TokenMethods };

use crate::data::token_defs::*;

use super::atp_processor::{ AtpProcessor, AtpProcessorMethods };

pub struct Builder {
    tokens: Vec<AtpToken>,
}
impl Builder {
    pub fn new() -> Builder {
        Builder {
            tokens: Vec::new(),
        }
    }

    pub fn build(&self) -> (AtpProcessor, String) {
        let mut processor = AtpProcessor::new();

        let identifier = processor.add_transform(&self.tokens);

        (processor, identifier)
    }
}

impl Builder {
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
    pub fn add_to_end(mut self, text: String) -> Self {
        self.tokens.push(AtpToken::Ate(ate::Ate { text: text }));
        self
    }
    pub fn add_to_beginning(mut self, text: String) -> Self {
        self.tokens.push(AtpToken::Atb(atb::Atb { text: text }));
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
    pub fn replace_all_with(mut self, pattern: String, text_to_replace: String) -> Self {
        self.tokens.push(AtpToken::Raw(raw::Raw { pattern, text_to_replace }));
        self
    }
    pub fn replace_first_with(mut self, pattern: String, text_to_replace: String) -> Self {
        self.tokens.push(AtpToken::Rfw(rfw::Rfw { pattern, text_to_replace }));
        self
    }
    pub fn rotate_left(mut self, times: i32) {
        self.tokens.push(AtpToken::Rtl(rtl::Rtl { times }));
    }
    pub fn rotate_right(mut self, times: i32) {
        self.tokens.push(AtpToken::Rtr(rtr::Rtr { times }));
    }
}
