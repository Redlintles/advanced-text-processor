#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::context::execution_context::GlobalExecutionContext;
use crate::tokens::InstructionMethods;
use crate::parser::params::TextForgeParamTypes;
use crate::utils::validations::check_vec_len;

use crate::utils::errors::{TextForgeError, TextForgeErrorCode};

/// RTL - Rotate Left
///
/// Rotates `input` to the left `n` times
///
/// # Example
///
/// ```rust
/// use textforge::tokens::{InstructionMethods, transforms::rtl::Rtl};
///
/// let token = Rtl::new(3);
///
/// assert_eq!(token.transform("banana", None), Ok("anaban".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Rtl {
    pub times: usize,
    params: Vec<TextForgeParamTypes>,
}

impl Rtl {
    pub fn new(times: usize) -> Rtl {
        Rtl {
            times,
            params: vec![times.into()],
        }
    }
}

impl InstructionMethods for Rtl {
    fn get_params(&self) -> &Vec<TextForgeParamTypes> {
        &self.params
    }
    fn transform(
        &self,
        input: &str,
        _: Option<&mut GlobalExecutionContext>,
    ) -> Result<String, TextForgeError> {
        if input.is_empty() {
            return Err(TextForgeError::new(
                TextForgeErrorCode::InvalidParameters("Input is empty".into()),
                self.to_textforge_line(),
                "\" \"",
            ));
        }

        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let times = self.times % len;

        Ok(chars[times..].iter().chain(&chars[..times]).collect())
    }

    fn to_textforge_line(&self) -> Cow<'static, str> {
        format!("rtl {};\n", self.times).into()
    }

    fn get_string_repr(&self) -> &'static str {
        "rtl"
    }
    fn from_params(&mut self, params: &Vec<TextForgeParamTypes>) -> Result<(), TextForgeError> {
        use crate::parse_args;

        check_vec_len(params, 1, "rtl", "")?;

        self.times = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.params = vec![self.times.into()];

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0e
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, TextForgeError> {
        use crate::to_bytecode;
        let result: Vec<u8> =
            to_bytecode!(self.get_opcode(), [TextForgeParamTypes::Usize(self.times)]);
        Ok(result)
    }
}
