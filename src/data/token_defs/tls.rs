use crate::data::{ TokenMethods };
// Trim left side
#[derive(Clone, Copy)]
pub struct Tls {}

impl TokenMethods for Tls {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        // "tls;"

        if line[0] == "tls" {
            return Ok(Tls {});
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> Self {
        Tls {}
    }

    fn token_to_atp_line(&self) -> String {
        "tls;\n".to_string()
    }

    fn get_string_repr() -> String {
        "tls".to_string()
    }
    fn parse(&self, input: &str) -> String {
        String::from(input.trim_start())
    }
}
