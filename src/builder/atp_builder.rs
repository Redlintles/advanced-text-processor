use crate::data::{ AtpToken, TokenMethods };

use crate::data::token_defs::ate::Ate;
use crate::data::token_defs::atb::Atb;
use crate::data::token_defs::trs::Trs;
use crate::data::token_defs::tls::Tls;
use crate::data::token_defs::tbs::Tbs;
use crate::data::token_defs::raw::Raw;
use crate::data::token_defs::rfw::Rfw;
use crate::data::token_defs::dlf::Dlf;
use crate::data::token_defs::dll::Dll;
use crate::data::token_defs::dlb::Dlb;
use crate::data::token_defs::dla::Dla;
use crate::data::token_defs::dlc::Dlc;

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
        self.tokens.push(Tbs::new());
        self
    }

    pub fn trim_left(mut self) -> Self {
        self.tokens.push(AtpToken::Tls(Tls {}));
        self
    }
    pub fn trim_right(mut self) -> Self {
        self.tokens.push(AtpToken::Trs(Trs {}));
        self
    }
    pub fn add_to_end(mut self, text: String) -> Self {
        self.tokens.push(AtpToken::Ate(Ate { text: text }));
        self
    }
    pub fn add_to_beginning(mut self, text: String) -> Self {
        self.tokens.push(AtpToken::Atb(Atb { text: text }));
        self
    }
    pub fn delete_first(mut self) -> Self {
        self.tokens.push(AtpToken::Dlf(Dlf {}));
        self
    }
    pub fn delete_last(mut self) -> Self {
        self.tokens.push(AtpToken::Dll(Dll {}));
        self
    }
    pub fn delete_after(mut self, index: usize) -> Self {
        self.tokens.push(AtpToken::Dla(Dla { index: index }));
        self
    }
    pub fn delete_before(mut self, index: usize) -> Self {
        self.tokens.push(AtpToken::Dlb(Dlb { index: index }));
        self
    }
    pub fn delete_chunk(mut self, start_index: usize, end_index: usize) -> Self {
        self.tokens.push(AtpToken::Dlc(Dlc { start_index: start_index, end_index: end_index }));
        self
    }
    pub fn replace_all_with(mut self, pattern: String, text_to_replace: String) -> Self {
        self.tokens.push(AtpToken::Raw(Raw { pattern: pattern, text_to_replace: text_to_replace }));
        self
    }
    pub fn replace_first_with(mut self, pattern: String, text_to_replace: String) -> Self {
        self.tokens.push(AtpToken::Rfw(Rfw { pattern: pattern, text_to_replace: text_to_replace }));
        self
    }
}
