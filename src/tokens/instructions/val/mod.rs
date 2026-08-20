use crate::{
    context::execution_context::{
        GlobalContextMethods,
        GlobalExecutionContext,
        VarEntry,
        VarValues,
    },
    globals::var::TokenWrapper,
    parse_args,
    to_bytecode,
    tokens::InstructionMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode::RequiredContextError },
        params::AtpParamTypes,
        validations::check_vec_len,
    },
};

#[cfg(feature = "test_access")]
pub mod test;
#[derive(Clone)]
pub struct Val {
    val_name: String,
    val_value: AtpParamTypes,
    params: Vec<AtpParamTypes>,
}

impl Default for Val {
    fn default() -> Self {
        Val {
            val_name: "x".to_string(),
            val_value: AtpParamTypes::String("".to_string()),
            params: vec![
                AtpParamTypes::String("x".to_string()),
                AtpParamTypes::Token(TokenWrapper::default())
            ],
        }
    }
}

impl InstructionMethods for Val {
    fn get_params(&self) -> &Vec<AtpParamTypes> {
        return &self.params;
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x36
    }
    fn get_string_repr(&self) -> &'static str {
        "val".into()
    }

    fn to_atp_line(&self) -> std::borrow::Cow<'static, str> {
        format!("val {} = {}", self.val_name, self.val_value.to_string()).into()
    }

    fn transform(
        &self,
        input: &str,
        context: Option<&mut GlobalExecutionContext>
    ) -> Result<String, crate::utils::errors::AtpError> {
        let context = context.ok_or_else(||
            AtpError::new(
                RequiredContextError("Context required for proper working!".into()),
                std::borrow::Cow::Borrowed("val"),
                std::borrow::Cow::Borrowed("")
            )
        )?;
        let value = match &self.val_value {
            AtpParamTypes::VarRef(name) => context.get_var(name)?.value.clone(),
            other => VarValues::try_from(other.clone())?,
        };

        context.add_var(&self.val_name, VarEntry {
            value,
            mutable: false,
        })?;

        Ok(input.to_string())
    }

    fn from_params(
        &mut self,
        params: &Vec<crate::utils::params::AtpParamTypes>
    ) -> Result<(), crate::utils::errors::AtpError> {
        check_vec_len(&params, 2, "val", "param parsing error, invalid vec len")?;

        self.val_name = parse_args!(params, 0, String, "Val name should be of string type");
        self.val_value = params[1].clone();

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, AtpError> {
        let result = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.val_name.clone()),
            self.val_value.clone(),
        ]);
        Ok(result)
    }
}
