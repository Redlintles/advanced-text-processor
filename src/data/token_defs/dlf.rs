use crate::data::{ AtpToken, TokenMethods };

// Delete first
#[derive(Clone, Copy)]
pub struct Dlf {}

impl TokenMethods for Dlf {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "dlf;"

        if line[0] == "dlf" {
            return Ok(AtpToken::Dlf(Dlf {}));
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Dlf(Dlf {})
    }

    fn token_to_atp_line(&self) -> String {
        "dlf;\n".to_string()
    }

    fn get_string_repr() -> String {
        "dlf".to_string()
    }
}
