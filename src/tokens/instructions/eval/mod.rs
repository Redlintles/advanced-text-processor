use std::borrow::Cow;
use std::collections::HashMap;

use evalexpr::{ContextWithMutableVariables, HashMapContext, Value, eval_with_context};

use crate::{
    context::execution_context::{
        GlobalContextMethods, GlobalExecutionContext, VarEntry, VarValues,
    },
    parse_args,
    parser::params::TextForgeParamTypes,
    tokens::InstructionMethods,
    utils::{
        errors::{
            TextForgeError,
            TextForgeErrorCode::{InvalidExprError, RequiredContextError},
        },
        validations::check_vec_len,
    },
};

#[cfg(feature = "test_access")]
pub mod test;

/// Builds a snapshot evalexpr context from the current variable table, so
/// expressions can reference stored variables by name.
///
/// Numeric conversion is a resolution-time concern only: everything in
/// `GlobalExecutionContext` stays `VarValues::String`/`Usize` before and
/// after `eval` runs — this function never mutates the context, it only
/// reads from it to build a disposable `HashMapContext`. Each variable is
/// tried as an `i64`; if it doesn't parse, it's passed to evalexpr as a
/// plain string. Floats aren't attempted on purpose: ATP's smallest
/// logical unit is a character, always represented as an integer, so
/// float support isn't worth the extra ambiguity.
///
/// Token variables are skipped: they aren't representable as evalexpr
/// values. Referencing one in an expression will surface as evalexpr's own
/// "identifier not found" error, which is an acceptable failure mode here.
fn build_eval_context(vars: &HashMap<String, VarEntry>) -> Result<HashMapContext, TextForgeError> {
    let mut ctx = HashMapContext::new();

    for (name, entry) in vars.iter() {
        let value = match &entry.value {
            VarValues::Usize(n) => Value::from_int(*n as i64),
            VarValues::String(s) => match s.parse::<i64>() {
                Ok(i) => Value::from_int(i),
                Err(_) => Value::from(s.clone()),
            },
            VarValues::Token(_) => {
                continue;
            }
        };

        ctx.set_value(name.clone(), value).map_err(|e| {
            TextForgeError::new(
                InvalidExprError(Cow::from(e.to_string())),
                Cow::from("eval.build_eval_context"),
                Cow::from(name.clone()),
            )
        })?;
    }

    Ok(ctx)
}

/// Converts an evalexpr result back into a plain string, without the
/// formatting evalexpr's own `Display` impl adds (e.g. wrapping strings in
/// literal quotes, since that impl is meant to print back valid expression
/// syntax, not a clean value).
fn value_to_plain_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Tuple(t) => {
            let parts: Vec<String> = t.iter().map(value_to_plain_string).collect();
            format!("({})", parts.join(", "))
        }
        Value::Empty => String::new(),
    }
}

/// Eval - Parses logical and
#[derive(Clone, Default)]
pub struct Eval {
    expr: String,
    target: String,
    params: Vec<TextForgeParamTypes>,
}

impl InstructionMethods for Eval {
    fn get_params(&self) -> &Vec<TextForgeParamTypes> {
        &self.params
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x41
    }
    fn get_string_repr(&self) -> &'static str {
        "eval"
    }

    fn to_textforge_line(&self) -> Cow<'static, str> {
        Cow::from(format!("eval {} in {};\n", self.expr, self.target))
    }

    fn transform(
        &self,
        input: &str,
        context: Option<&mut GlobalExecutionContext>,
    ) -> Result<String, TextForgeError> {
        let context = context.ok_or_else(|| {
            TextForgeError::new(
                RequiredContextError("Context required for proper working!".into()),
                std::borrow::Cow::Borrowed("val"),
                std::borrow::Cow::Borrowed(""),
            )
        })?;

        // Immutable borrow — dropped as soon as eval_ctx is built, since
        // HashMapContext owns its own converted values rather than
        // referencing context's HashMap.
        let eval_ctx = build_eval_context(context.get_all_vars())?;

        let result = eval_with_context(&self.expr, &eval_ctx).map_err(|e| {
            TextForgeError::new(
                InvalidExprError(Cow::from(e.to_string())),
                Cow::from("eval.transform"),
                Cow::from(self.expr.to_string()),
            )
        })?;

        let var_mut = context.get_mut_var(&self.target)?;

        if var_mut.mutable {
            var_mut.value = VarValues::String(value_to_plain_string(&result));
        }
        Ok(input.to_string())
    }

    fn from_params(&mut self, params: &Vec<TextForgeParamTypes>) -> Result<(), TextForgeError> {
        check_vec_len(params, 2, "eval", "param parsing error, invalid vec len")?;

        self.expr = parse_args!(params, 0, String, "Expr should be of type string");
        self.target = parse_args!(params, 1, String, " should be of type string");

        self.params = vec![self.expr.to_string().into(), self.target.to_string().into()];
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Result<Vec<u8>, TextForgeError> {
        use crate::to_bytecode;
        let result = to_bytecode!(self.get_opcode(), []);
        Ok(result)
    }
}
