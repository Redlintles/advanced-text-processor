use regex::Regex;

use crate::data::{ AtpToken, TokenMethods };

// Replace first with
#[derive(Clone)]
pub struct Rfw {
    pub pattern: String,
    pub text_to_replace: String,
}

impl Rfw {
    fn params(pattern: String, text_to_replace: String) -> Self {
        Rfw {
            pattern,
            text_to_replace,
        }
    }
}

impl TokenMethods for Rfw {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "rfw;"

        if line[0] == "rfw" {
            return Ok(AtpToken::Rfw(Rfw::params(line[1].clone(), line[2].clone())));
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Rfw(Rfw { pattern: "".to_string(), text_to_replace: "".to_string() })
    }

    fn token_to_atp_line(&self) -> String {
        format!("rfw {} {};\n", self.pattern, self.text_to_replace)
    }

    fn get_string_repr() -> String {
        "rfw".to_string()
    }
    fn parse(&self, input: &str) -> String {
        let re = Regex::new(&self.pattern).unwrap();

        let result = re.replace(input, &self.text_to_replace);

        result.to_string()
    }
}
