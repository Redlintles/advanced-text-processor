use crate::data::{ TokenMethods };

// Trim both sides
#[derive(Clone)]
pub struct Tbs {}

impl Tbs {
    pub fn new() -> Self {
        Tbs {}
    }
}

impl TokenMethods for Tbs {
    fn token_to_atp_line(&self) -> String {
        "tbs;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        String::from(input.trim())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "tbs;"

        if line[0] == "tbs" {
            return Ok(());
        }
        Err("Erro de parsing".to_string())
    }

    fn get_string_repr(&self) -> String {
        "tbs".to_string()
    }
}
