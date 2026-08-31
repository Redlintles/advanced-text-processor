use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{errors::TextForgeError, params::TextForgeParamTypes, validations::check_vec_len},
};

#[cfg(feature = "test_access")]
pub mod test;

/// Null - Does nothing
#[derive(Clone, Default)]
pub struct Null {
    params: Vec<TextForgeParamTypes>,
}

impl InstructionMethods for Null {
    fn get_params(&self) -> &Vec<TextForgeParamTypes> {
        &self.params
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x36
    }
    fn get_string_repr(&self) -> &'static str {
        "null"
    }

    fn to_textforge_line(&self) -> Cow<'static, str> {
        Cow::from("null;\n".to_string())
    }

    fn transform(
        &self,
        input: &str,
        _: Option<&mut GlobalExecutionContext>,
    ) -> Result<String, TextForgeError> {
        Ok(input.to_string())
    }

    fn from_params(&mut self, params: &Vec<TextForgeParamTypes>) -> Result<(), TextForgeError> {
        check_vec_len(params, 0, "null", "param parsing error, invalid vec len")?;

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, TextForgeError> {
        use crate::to_bytecode;
        let result = to_bytecode!(self.get_opcode(), []);
        Ok(result)
    }
}
