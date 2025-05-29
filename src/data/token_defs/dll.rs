use crate::data::{ TokenMethods };

// Delete last
#[derive(Clone, Copy)]
pub struct Dll {}

impl Dll {
    pub fn new() -> Self {
        Dll {}
    }
}

impl TokenMethods for Dll {
    fn token_to_atp_line(&self) -> String {
        "dll;\n".to_string()
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);

        if let Some((x, _)) = s.char_indices().rev().next() {
            s.drain(x..);
        }

        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "dll;"

        if line[0] == "dll" {
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "dll".to_string()
    }
}
