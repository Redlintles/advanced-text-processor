#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{
        errors::{TextForgeError, TextForgeErrorCode},
        validations::check_vec_len,
    },
};

use crate::utils::params::TextForgeParamTypes;
/// RTR - Rotate Right
///
/// Rotates `input` to the right `n` times
///
/// # Example
///
/// ```rust
/// use textforge::tokens::{InstructionMethods, transforms::rtr::Rtr};
///
/// let token = Rtr::new(2);
///
/// assert_eq!(token.transform("banana", None), Ok("nabana".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Rtr {
    pub times: usize,
    params: Vec<TextForgeParamTypes>,
}

impl Rtr {
    pub fn new(times: usize) -> Rtr {
        Rtr {
            times,
            params: vec![times.into()],
        }
    }
}

impl InstructionMethods for Rtr {
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

        Ok(chars[len - times..]
            .iter()
            .chain(&chars[..len - times])
            .collect())
    }

    fn to_textforge_line(&self) -> Cow<'static, str> {
        format!("rtr {};\n", self.times).into()
    }
    fn get_string_repr(&self) -> &'static str {
        "rtr"
    }

    fn from_params(&mut self, params: &Vec<TextForgeParamTypes>) -> Result<(), TextForgeError> {
        use crate::parse_args;

        check_vec_len(params, 1, "rtr", "")?;

        self.times = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.params = vec![self.times.into()];

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0f
    }

    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, TextForgeError> {
        use crate::to_bytecode;
        let result: Vec<u8> =
            to_bytecode!(self.get_opcode(), [TextForgeParamTypes::Usize(self.times)]);
        Ok(result)
    }
}
