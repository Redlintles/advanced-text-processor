use crate::data::{ TokenMethods };
// Delete first
#[derive(Clone, Copy)]
pub struct Dlf {}

impl TokenMethods for Dlf {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        // "dlf;"

        if line[0] == "dlf" {
            return Ok(Dlf {});
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> Self {
        Dlf {}
    }

    fn token_to_atp_line(&self) -> String {
        "dlf;\n".to_string()
    }

    fn get_string_repr() -> String {
        "dlf".to_string()
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);
        s.drain(..1);
        s
    }
}
