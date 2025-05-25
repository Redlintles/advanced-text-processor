use crate::data::{ AtpToken, TokenMethods };

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
}

impl TokenMethods for Dlb {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "dlb;"

        if line[0] == "dlb" {
            return Ok(
                AtpToken::Dlb(
                    Dlb::params(line[1].clone().parse().expect("Parse from string to usize failed"))
                )
            );
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Dlb(Dlb { index: 0 as usize })
    }

    fn token_to_atp_line(&self) -> String {
        format!("dlb {};\n", self.index)
    }

    fn get_string_repr() -> String {
        "dlb".to_string()
    }
}
