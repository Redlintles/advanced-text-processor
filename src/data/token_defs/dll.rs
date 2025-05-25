use crate::data::{ TokenMethods, AtpToken };

// Delete last
#[derive(Clone, Copy)]
pub struct Dll {}

impl TokenMethods for Dll {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "dll;"

        if line[0] == "dll" {
            return Ok(AtpToken::Dll(Dll {}));
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Dll(Dll {})
    }

    fn token_to_atp_line(&self) -> String {
        "dll;\n".to_string()
    }

    fn get_string_repr() -> String {
        "dll".to_string()
    }
}
