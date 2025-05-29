use crate::data::{ TokenMethods };
#[derive(Clone)]
pub struct Rpt {
    pub times: usize,
}

impl Rpt {
    pub fn params(times: usize) -> Self {
        Rpt { times }
    }

    pub fn new() -> Self {
        return Rpt {
            times: 0,
        };
    }
}

impl TokenMethods for Rpt {
    fn token_to_atp_line(&self) -> String {
        format!("rpt {};\n", self.times)
    }

    fn parse(&self, input: &str) -> String {
        input.repeat(self.times)
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        if line[0] == "rpt" {
            self.times = line[1].parse().expect("Parsing from string to usize failed");
            return Ok(());
        }

        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "rpt".to_string()
    }
}
