use crate::data::{ TokenMethods };

// Trim right side
#[derive(Clone, Copy)]
pub struct Trs {}

impl TokenMethods for Trs {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        // "trs;"

        if line[0] == "trs" {
            return Ok(Trs {});
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> Self {
        Trs {}
    }

    fn token_to_atp_line(&self) -> String {
        "trs;\n".to_string()
    }

    fn get_string_repr() -> String {
        "trs".to_string()
    }

    fn parse(&self, input: &str) -> String {
        String::from(input.trim_end())
    }
}
