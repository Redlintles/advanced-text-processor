use crate::data::{ TokenFactory, TokenMethods };

// Trim both sides
#[derive(Clone)]
pub struct Tbs {}

impl TokenMethods for Tbs {
    fn token_to_atp_line(&self) -> String {
        "tbs;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        String::from(input.trim())
    }
}

impl TokenFactory<Tbs> for Tbs {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        // "tbs;"

        if line[0] == "tbs" {
            return Ok(Tbs {});
        }
        Err("Erro de parsing".to_string())
    }

    fn new() -> Self {
        Tbs {}
    }

    fn get_string_repr() -> String {
        "tbs".to_string()
    }
}
