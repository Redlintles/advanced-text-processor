use crate::data::{ TokenMethods };

// Trim right side
#[derive(Clone, Copy)]
pub struct Trs {}

impl Trs {
    pub fn new() -> Self {
        Trs {}
    }
}

impl TokenMethods for Trs {
    fn token_to_atp_line(&self) -> String {
        "trs;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        String::from(input.trim_end())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "trs;"

        if line[0] == "trs" {
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "trs".to_string()
    }
}
