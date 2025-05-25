use crate::data::{ TokenMethods, AtpToken };

// Trim right side
#[derive(Clone, Copy)]
pub struct Trs {}

impl TokenMethods for Trs {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "trs;"

        if line[0] == "trs" {
            return Ok(AtpToken::Trs(Trs {}));
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Trs(Trs {})
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
