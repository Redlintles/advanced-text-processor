use crate::data::{ AtpToken, TokenMethods };

// Delete after
#[derive(Clone, Copy)]
pub struct Dla {
    pub index: usize,
}

impl Dla {
    fn params(index: usize) -> Self {
        Dla {
            index,
        }
    }
}

impl TokenMethods for Dla {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "dla;"

        if line[0] == "dla" {
            return Ok(
                AtpToken::Dla(
                    Dla::params(line[1].clone().parse().expect("Parse from string to usize failed"))
                )
            );
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Dla(Dla { index: 0 as usize })
    }

    fn token_to_atp_line(&self) -> String {
        format!("dla {};\n", self.index)
    }

    fn get_string_repr() -> String {
        "dla".to_string()
    }
}
