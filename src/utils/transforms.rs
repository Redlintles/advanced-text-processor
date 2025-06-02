pub fn string_to_usize(chunk: &str) -> Result<usize, String> {
    match chunk.parse() {
        Ok(v) => Ok(v),
        Err(_) => { Err("Parsing from string to usize failed".to_string()) }
    }
}

#[cfg(feature = "bytecode")]
pub fn token_from_hex_string(s: &str) -> Option<u8> {
    let stripped = s.strip_prefix("0x")?;

    let byte = u8::from_str_radix(stripped, 16).ok()?;

    Some(byte)
}
#[cfg(feature = "bytecode")]
use crate::{
    bytecode_parser::BytecodeTokenMethods,
    token_data::TokenMethods,
    utils::mapping::get_supported_bytecode_tokens,
};

#[cfg(feature = "bytecode")]
pub fn token_to_bytecode_token(
    token: &Box<dyn TokenMethods>
) -> Result<Box<dyn BytecodeTokenMethods>, String> {
    let mut line = token.token_to_atp_line();

    line.pop();

    let chunks = match shell_words::split(&line) {
        Ok(x) => x,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let string_token_map = get_supported_bytecode_tokens();

    let factory = match string_token_map.get(&token.get_string_repr()) {
        Some(x) => x,
        None => {
            return Err("Invalid Token".to_string());
        }
    };

    let mut new_token = factory();

    new_token.token_from_vec_params(chunks)?;

    Ok(new_token)
}

#[cfg(feature = "bytecode")]
pub fn bytecode_token_to_token(
    token: &Box<dyn BytecodeTokenMethods>
) -> Result<Box<dyn TokenMethods>, String> {
    use super::mapping::get_supported_default_tokens;

    let mut line = token.token_to_atp_line();

    line.pop();

    let chunks = shell_words::split(&line).map_err(|e| e.to_string())?;

    let string_token_map = get_supported_default_tokens();

    let factory = match string_token_map.get(&token.get_string_repr()) {
        Some(x) => x,
        None => {
            return Err("Invalid Token".to_string());
        }
    };

    let mut new_token = factory();

    new_token.token_from_vec_params(chunks)?;

    Ok(new_token)
}

#[cfg(feature = "bytecode")]
pub fn token_vec_to_bytecode_token_vec(
    tokens: &Vec<Box<dyn TokenMethods>>
) -> Result<Vec<Box<dyn BytecodeTokenMethods>>, String> {
    let mut r: Vec<Box<dyn BytecodeTokenMethods>> = Vec::new();

    for tk in tokens {
        let parsed_token = token_to_bytecode_token(tk)?;

        r.push(parsed_token);
    }
    Ok(r)
}
#[cfg(feature = "bytecode")]
pub fn bytecode_token_vec_to_token_vec(
    tokens: &Vec<Box<dyn BytecodeTokenMethods>>
) -> Result<Vec<Box<dyn TokenMethods>>, String> {
    let mut r: Vec<Box<dyn TokenMethods>> = Vec::new();

    for tk in tokens {
        let parsed_token = bytecode_token_to_token(tk)?;

        r.push(parsed_token);
    }
    Ok(r)
}
