#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::TextForgeError, validations::check_vec_len },
};

use crate::parser::params::TextForgeParamTypes;

/// RPT - Repeat
///
/// Repeats `input` n `times`
///
/// # Example
///
/// ```rust
/// use textforge::tokens::{InstructionMethods, transforms::rpt::Rpt};
///
/// let token = Rpt::new(3);
///
/// assert_eq!(token.transform("banana".into(),None).unwrap().to_string(), "bananabananabanana");
///
/// ```
#[derive(Clone, Default)]
pub struct Rpt {
    pub times: usize,
    params: Vec<TextForgeParamTypes>,
}

impl Rpt {
    pub fn new(times: usize) -> Self {
        Rpt {
            times,
            params: vec![times.into()],
        }
    }
}

impl InstructionMethods for Rpt {
    fn get_params(&self) -> &Vec<TextForgeParamTypes> {
        &self.params
    }
    fn to_textforge_line(&self) -> Cow<'static, str> {
        format!("rpt {};\n", self.times).into()
    }

    fn transform<'a>(
        &self,
        input: Cow<'a, str>,
        _: Option<&mut GlobalExecutionContext>
    ) -> Result<Cow<'a, str>, TextForgeError> {
        Ok(input.repeat(self.times).into())
    }

    fn get_string_repr(&self) -> &'static str {
        "rpt"
    }
    fn from_params(&mut self, params: &Vec<TextForgeParamTypes>) -> Result<(), TextForgeError> {
        use crate::parse_args;

        check_vec_len(params, 1, "rpt", "")?;

        self.times = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.params = vec![self.times.into()];

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0d
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, TextForgeError> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            TextForgeParamTypes::Usize(self.times),
        ]);
        Ok(result)
    }
}
