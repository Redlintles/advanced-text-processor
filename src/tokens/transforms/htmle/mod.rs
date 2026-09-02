#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use html_escape::encode_safe;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{errors::TextForgeError, validations::check_vec_len},
};

use crate::parser::params::TextForgeParamTypes;

/// HTMLE - HTML Escape
///
/// Escapes Special HTML Characters in `input` to HTML Entities
/// So they can be rendered correctly later
///
/// # Example
///
/// ```rust
/// use textforge::tokens::{InstructionMethods, transforms::htmle::Htmle};
///
/// let token = Htmle::default();
///
/// assert_eq!(token.transform("<div>banana</div>", None), Ok("&lt;div&gt;banana&lt;&#x2F;div&gt;".to_string()));
/// ```

#[derive(Clone, Default)]
pub struct Htmle {
    params: Vec<TextForgeParamTypes>,
}

impl InstructionMethods for Htmle {
    fn get_params(&self) -> &Vec<TextForgeParamTypes> {
        &self.params
    }
    fn get_string_repr(&self) -> &'static str {
        "htmle"
    }

    fn to_textforge_line(&self) -> Cow<'static, str> {
        "htmle;\n".into()
    }
    fn transform(
        &self,
        input: &str,
        _: Option<&mut GlobalExecutionContext>,
    ) -> Result<String, TextForgeError> {
        Ok(encode_safe(input).to_string())
    }
    fn from_params(&mut self, params: &Vec<TextForgeParamTypes>) -> Result<(), TextForgeError> {
        check_vec_len(params, 0, "dlf", "")?;
        Ok(())
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x24
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, TextForgeError> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        Ok(result)
    }
}
