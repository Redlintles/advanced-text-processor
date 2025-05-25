use crate::data::{ TokenMethods };

// Delete last
#[derive(Clone, Copy)]
pub struct Dll {}

impl TokenMethods for Dll {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        // "dll;"

        if line[0] == "dll" {
            return Ok(Dll {});
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> Self {
        Dll {}
    }

    fn token_to_atp_line(&self) -> String {
        "dll;\n".to_string()
    }

    fn get_string_repr() -> String {
        "dll".to_string()
    }
    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);

        if let Some((x, _)) = s.char_indices().rev().next() {
            s.drain(x..);
        }

        s
    }
}
