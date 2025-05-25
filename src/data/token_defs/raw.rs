use crate::data::{ AtpToken, TokenMethods };

// Replace all with
#[derive(Clone)]
pub struct Raw {
    pub pattern: String,
    pub text_to_replace: String,
}

impl Raw {
    fn params(pattern: String, text_to_replace: String) -> Self {
        Raw {
            pattern,
            text_to_replace,
        }
    }
}

impl TokenMethods for Raw {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "raw;"

        if line[0] == "raw" {
            return Ok(AtpToken::Raw(Raw::params(line[1].clone(), line[2].clone())));
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Raw(Raw { pattern: "".to_string(), text_to_replace: "".to_string() })
    }

    fn token_to_atp_line(&self) -> String {
        format!("raw {} {};\n", self.pattern, self.text_to_replace)
    }

    fn get_string_repr() -> String {
        "raw".to_string()
    }
}
