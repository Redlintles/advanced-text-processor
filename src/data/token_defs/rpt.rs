use crate::data::TokenMethods;
#[derive(Clone)]
pub struct Rpt {
    pub times: usize,
}

impl Rpt {
    pub fn params(times: usize) -> Self {
        Rpt { times }
    }
}

impl TokenMethods for Rpt {
    fn get_string_repr() -> String {
        "rpt".to_string()
    }
    fn new() -> Self {
        Rpt {
            times: 0,
        }
    }
    fn token_to_atp_line(&self) -> String {
        format!("rpt {};\n", self.times)
    }

    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        if line[0] == "rpt" {
            return Ok(Rpt { times: line[1].parse().expect("Parsing from string to usize failed") });
        }

        Err("Parsing Error".to_string())
    }

    fn parse(&self, input: &str) -> String {
        input.repeat(self.times)
    }
}
