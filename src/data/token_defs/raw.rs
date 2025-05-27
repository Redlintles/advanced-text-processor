use regex::Regex;

use crate::data::{ TokenMethods };
// Replace all with
#[derive(Clone)]
pub struct Raw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Raw {
    fn params(pattern: String, text_to_replace: String) -> Self {
        Raw {
            text_to_replace,
            pattern: Regex::new(&pattern).expect("Failed creating regex"),
        }
    }
}

impl TokenMethods for Raw {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        // "raw;"

        if line[0] == "raw" {
            return Ok(Raw::params(line[1].clone(), line[2].clone()));
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> Self {
        Raw {
            pattern: Regex::new("").expect("Failed creating regex"),
            text_to_replace: "".to_string(),
        }
    }

    fn token_to_atp_line(&self) -> String {
        format!("raw {} {};\n", self.pattern, self.text_to_replace)
    }

    fn get_string_repr() -> String {
        "raw".to_string()
    }

    fn parse(&self, input: &str) -> String {
        self.pattern.replace_all(input, &self.text_to_replace).to_string()
    }
}
