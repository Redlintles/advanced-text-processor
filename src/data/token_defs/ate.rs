use crate::data::{ TokenMethods };
// add to end
#[derive(Clone)]
pub struct Ate {
    pub text: String,
}

impl Ate {
    fn params(text: String) -> Self {
        Ate {
            text,
        }
    }
}

impl TokenMethods for Ate {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        // "ate;"

        if line[0] == "ate" {
            return Ok(Ate::params(line[1].clone()));
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> Self {
        Ate { text: "".to_string() }
    }

    fn token_to_atp_line(&self) -> String {
        format!("ate {};\n", self.text)
    }

    fn get_string_repr() -> String {
        "ate".to_string()
    }
    fn parse(&self, input: &str) -> String {
        let mut s = String::from(&self.text);
        s.push_str(input);
        s
    }
}
