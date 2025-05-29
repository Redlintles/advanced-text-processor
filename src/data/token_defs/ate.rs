use crate::data::{ TokenMethods };
// add to end
#[derive(Clone)]
pub struct Ate {
    pub text: String,
}

impl Ate {
    pub fn params(text: String) -> Self {
        Ate {
            text,
        }
    }
    pub fn new() -> Self {
        Ate { text: "".to_string() }
    }
}

impl TokenMethods for Ate {
    fn token_to_atp_line(&self) -> String {
        format!("ate {};\n", self.text)
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(&self.text);
        s.push_str(input);
        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "ate;"

        if line[0] == "ate" {
            self.text = line[1].clone();
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "ate".to_string()
    }
}
