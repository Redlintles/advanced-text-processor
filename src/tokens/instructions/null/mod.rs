use std::borrow::Cow;

use crate::{
    context::execution_context::{ GlobalExecutionContext },
    to_bytecode,
    tokens::InstructionMethods,
    utils::{ errors::{ TextForgeError }, params::TextForgeParamTypes, validations::check_vec_len },
};

#[cfg(feature = "test_access")]
pub mod test;

/// Null - Does nothing
#[derive(Clone)]
pub struct Null {
    params: Vec<TextForgeParamTypes>,
}

impl Default for Null {
    fn default() -> Self {
        Null {
            params: vec![],
        }
    }
}

impl InstructionMethods for Null {
    fn get_params(&self) -> &Vec<TextForgeParamTypes> {
        return &self.params;
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x36
    }
    fn get_string_repr(&self) -> &'static str {
        "val".into()
    }

    fn to_textforge_line(&self) -> Cow<'static, str> {
        Cow::from(format!("null;\n"))
    }

    fn transform(
        &self,
        input: &str,
        _: Option<&mut GlobalExecutionContext>
    ) -> Result<String, crate::utils::errors::TextForgeError> {
        Ok(input.to_string())
    }

    fn from_params(
        &mut self,
        params: &Vec<crate::utils::params::TextForgeParamTypes>
    ) -> Result<(), crate::utils::errors::TextForgeError> {
        check_vec_len(&params, 0, "null", "param parsing error, invalid vec len")?;

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, TextForgeError> {
        let result = to_bytecode!(self.get_opcode(), []);
        Ok(result)
    }
}
