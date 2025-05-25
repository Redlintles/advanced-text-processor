use crate::data::{ AtpToken, TokenMethods };

// Delete Chunk
#[derive(Clone, Copy)]
pub struct Dlc {
    pub start_index: usize,
    pub end_index: usize,
}

impl Dlc {
    fn params(start_index: usize, end_index: usize) -> Self {
        Dlc {
            start_index,
            end_index,
        }
    }
}

impl TokenMethods for Dlc {
    fn token_from_vec_params(line: Vec<String>) -> Result<AtpToken, String> {
        // "dlc;"

        if line[0] == "dlc" {
            return Ok(
                AtpToken::Dlc(
                    Dlc::params(
                        line[1].clone().parse().expect("Parse from string to usize failed"),
                        line[2].clone().parse().expect("Parse from string to usize failed")
                    )
                )
            );
        }
        Err("Parsing Error".to_string())
    }

    fn new() -> AtpToken {
        AtpToken::Dlc(Dlc { start_index: 0 as usize, end_index: 0 as usize })
    }

    fn token_to_atp_line(&self) -> String {
        format!("dlc {} {};\n", self.start_index, self.end_index)
    }

    fn get_string_repr() -> String {
        "dlc".to_string()
    }
    fn parse(&self, input: &str) -> String {
        let start_index = input
            .char_indices()
            .nth(self.start_index)
            .map(|(i, _)| i)
            .unwrap_or(input.len());
        let end_index = input
            .char_indices()
            .nth(self.end_index)
            .map(|(i, _)| i)
            .unwrap_or(input.len());

        String::from(&input[start_index..end_index])
    }
}
