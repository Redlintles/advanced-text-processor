#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::context::execution_context::GlobalExecutionContext;
use crate::utils::params::TextForgeParamTypes;
use crate::{
    tokens::InstructionMethods,
    utils::{
        errors::TextForgeError,
        validations::{check_index_against_words, check_vec_len},
    },
};
/// TUCW - To Uppercase Word
///
/// Uppercase a single word of string
///
/// # Example:
///
/// ```rust
/// use textforge::tokens::{InstructionMethods, transforms::tucw::Tucw};
///
/// let token = Tucw::new(1);
///
/// assert_eq!(token.transform("banana laranja cheia de canja", None), Ok("banana LARANJA cheia de canja".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Tucw {
    index: usize,
    params: Vec<TextForgeParamTypes>,
}

impl Tucw {
    pub fn new(index: usize) -> Self {
        Tucw {
            index,
            params: vec![index.into()],
        }
    }
}
impl InstructionMethods for Tucw {
    fn get_params(&self) -> &Vec<TextForgeParamTypes> {
        &self.params
    }
    fn get_string_repr(&self) -> &'static str {
        "tucw"
    }

    fn to_textforge_line(&self) -> Cow<'static, str> {
        format!("tucw {};\n", self.index).into()
    }

    fn transform(
        &self,
        input: &str,
        _: Option<&mut GlobalExecutionContext>,
    ) -> Result<String, TextForgeError> {
        check_index_against_words(self.index, input)?;
        Ok(input
            .split_whitespace()
            .enumerate()
            .map(|(i, w)| {
                if i == self.index {
                    w.to_uppercase()
                } else {
                    w.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
            .to_string())
    }

    fn from_params(&mut self, params: &Vec<TextForgeParamTypes>) -> Result<(), TextForgeError> {
        use crate::parse_args;
        check_vec_len(params, 1, "tucw", "")?;

        self.index = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.params = vec![self.index.into()];

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x2a
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, TextForgeError> {
        use crate::to_bytecode;
        let result: Vec<u8> =
            to_bytecode!(self.get_opcode(), [TextForgeParamTypes::Usize(self.index)]);
        Ok(result)
    }
}
