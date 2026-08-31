#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{errors::TextForgeError, validations::check_vec_len},
};

use crate::utils::params::TextForgeParamTypes;
/// Token `Ate` — Add to End
///
/// Appends `text` to the end of `input`
///
/// # Example
///
/// ```rust
/// use textforge::tokens::{InstructionMethods, transforms::ate::Ate};
///
/// let token = Ate::new(" bar");
/// assert_eq!(token.transform("foo", None), Ok("foo bar".to_string()));
/// ```

#[derive(Clone, Default)]
pub struct Ate {
    pub text: String,
    params: Vec<TextForgeParamTypes>,
}

impl Ate {
    pub fn new(text: &str) -> Self {
        Ate {
            text: text.to_string(),
            params: vec![text.to_string().into()],
        }
    }
}

impl InstructionMethods for Ate {
    fn get_params(&self) -> &Vec<TextForgeParamTypes> {
        &self.params
    }
    fn to_textforge_line(&self) -> Cow<'static, str> {
        format!("ate {};\n", self.text).into()
    }

    fn transform(
        &self,
        input: &str,
        _: Option<&mut GlobalExecutionContext>,
    ) -> Result<String, TextForgeError> {
        let mut s = String::from(input);
        s.push_str(&self.text);
        Ok(s)
    }

    fn get_string_repr(&self) -> &'static str {
        "ate"
    }
    fn from_params(&mut self, params: &Vec<TextForgeParamTypes>) -> Result<(), TextForgeError> {
        use crate::parse_args;
        use crate::utils::params::TextForgeParamTypesJoin;

        check_vec_len(params, 1, "ate", params.join(""))?;

        self.text = parse_args!(params, 0, String, "Text should be of string type");

        self.params = vec![self.text.to_string().into()];

        Ok(())
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x02
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, TextForgeError> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(
            self.get_opcode(),
            [TextForgeParamTypes::String(self.text.clone()),]
        );
        Ok(result)
    }
}
