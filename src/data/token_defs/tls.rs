use crate::data::{ TokenMethods };
// Trim left side
#[derive(Clone, Copy)]
pub struct Tls {}
impl Tls {
    pub fn new() -> Self {
        Tls {}
    }
}

impl TokenMethods for Tls {
    fn token_to_atp_line(&self) -> String {
        "tls;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        String::from(input.trim_start())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "tls;"

        if line[0] == "tls" {
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }
    fn get_string_repr(&self) -> String {
        "tls".to_string()
    }
}
