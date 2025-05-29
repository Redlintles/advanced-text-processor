use crate::data::{ TokenMethods };
// Delete before
#[derive(Clone, Copy)]
pub struct Dlb {
    pub index: usize,
}

impl Dlb {
    fn params(index: usize) -> Self {
        Dlb {
            index,
        }
    }
    fn new() -> Self {
        Dlb { index: 0 }
    }
}

impl TokenMethods for Dlb {
    fn token_to_atp_line(&self) -> String {
        format!("dlb {};\n", self.index)
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);

        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index)
                .map(|(i, _)| i)
        {
            s.drain(..=byte_index);
        }

        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "dlb;"

        if line[0] == "dlb" {
            self.index = line[1].clone().parse().expect("Parse from string to usize failed");
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "dlb".to_string()
    }
}
