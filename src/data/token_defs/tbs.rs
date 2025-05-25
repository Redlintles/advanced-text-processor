use crate::data::{ TokenMethods, AtpToken };

// Trim both sides
#[derive(Clone)]
pub struct Tbs {}

impl TokenMethods for Tbs {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "tbs;"

        if line[0] == "tbs" {
            return Ok(AtpToken::Tbs(Tbs {}));
        }
        Err("Erro de parsing".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Tbs(Tbs {})
    }

    fn token_to_atp_line(&self) -> String {
        "tbs;\n".to_string()
    }

    fn get_string_repr() -> String {
        "tbs".to_string()
    }

    fn parse(&self, input: &str) -> String {
        String::from(input.trim())
    }
}
