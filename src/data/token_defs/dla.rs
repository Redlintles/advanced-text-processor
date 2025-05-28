use crate::data::{ TokenFactory, TokenMethods };
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
    fn token_to_atp_line(&self) -> String {
        format!("dla {};\n", self.index)
    }

    fn parse(&self, input: &str) -> String {
        let mut s = String::from(input);

        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index)
                .map(|(i, _)| i)
        {
            s.drain(byte_index..);
        }

        s
    }
}

impl TokenFactory<Dla> for Dla {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        // "dla;"

        if line[0] == "dla" {
            return Ok(
                Dla::params(line[1].clone().parse().expect("Parse from string to usize failed"))
            );
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> Self {
        Dla { index: 0 as usize }
    }

    fn get_string_repr() -> String {
        "dla".to_string()
    }
}
