use crate::data::{ AtpToken, TokenMethods };

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
}

impl TokenMethods for Atb {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "atb;"

        if line[0] == "atb" {
            return Ok(AtpToken::Atb(Atb::params(line[1].clone())));
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Atb(Atb { text: "".to_string() })
    }

    fn token_to_atp_line(&self) -> String {
        format!("atb {};\n", self.text)
    }

    fn get_string_repr() -> String {
        "atb".to_string()
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);
        s.push_str(&self.text);
        s
    }
}
