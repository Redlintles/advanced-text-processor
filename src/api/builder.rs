use crate::{
    api::{TextForgeBlockMethods, TextForgeBuilderMethods, TextForgeConditionalMethods},
    globals::var::TokenWrapper,
    utils::errors::TextForgeError,
};

use super::processor::{TextForgeProcessor, TextForgeProcessorMethods};

pub struct TextForgeBuilder<'ap> {
    tokens: Vec<TokenWrapper>,
    processor: &'ap mut TextForgeProcessor,
}

impl<'ap> TextForgeBuilder<'ap> {
    pub fn new(processor: &'ap mut TextForgeProcessor) -> TextForgeBuilder<'ap> {
        TextForgeBuilder {
            tokens: Vec::new(),
            processor,
        }
    }

    pub fn build(&mut self) -> String {
        let id = self.processor.add_transform(self.tokens.clone());

        id
    }
}

impl<'ap> TextForgeBuilderMethods for TextForgeBuilder<'ap> {
    fn push_token(&mut self, t: impl Into<TokenWrapper>) -> Result<(), TextForgeError> {
        self.tokens.push(t.into());
        Ok(())
    }
}

impl<'ap> TextForgeConditionalMethods for TextForgeBuilder<'ap> {}
impl<'ap> TextForgeBlockMethods for TextForgeBuilder<'ap> {}
