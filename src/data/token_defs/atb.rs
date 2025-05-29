use crate::data::{ TokenMethods };

// add to beginning
#[derive(Clone)]
pub struct Atb {
    pub text: String,
}

impl Atb {
    fn params(text: String) -> Self {
        Atb {
            text,
        }
    }

    fn new() -> Self {
        Atb { text: "".to_string() }
    }
}

impl TokenMethods for Atb {
    fn token_to_atp_line(&self) -> String {
        format!("atb {};\n", self.text)
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);
        s.push_str(&self.text);
        s
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "atb;"

        if line[0] == "atb" {
            self.text = line[1].clone();
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "atb".to_string()
    }
}
