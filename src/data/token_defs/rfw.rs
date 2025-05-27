use regex::Regex;

use crate::data::{ TokenMethods };
// Replace first with
#[derive(Clone)]
pub struct Rfw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Rfw {
    fn params(pattern: String, text_to_replace: String) -> Self {
        Rfw {
            pattern: Regex::new(&pattern).expect("Failed creating regex"),
            text_to_replace,
        }
    }
}

impl TokenMethods for Rfw {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        // "rfw;"

        if line[0] == "rfw" {
            return Ok(Rfw::params(line[1].clone(), line[2].clone()));
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> Self {
        Rfw {
            pattern: Regex::new("").expect("Failed creating regex"),
            text_to_replace: "".to_string(),
        }
    }

    fn token_to_atp_line(&self) -> String {
        format!("rfw {} {};\n", self.pattern, self.text_to_replace)
    }

    fn get_string_repr() -> String {
        "rfw".to_string()
    }
    fn parse(&self, input: &str) -> String {
        self.pattern.replace(input, &self.text_to_replace).to_string()
    }
}
