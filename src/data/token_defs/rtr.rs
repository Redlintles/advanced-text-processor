use crate::data::TokenMethods;
#[derive(Clone)]
pub struct Rtr {
    pub times: i32,
}

impl Rtr {
    fn params(times: i32) -> Rtr {
        Rtr {
            times,
        }
    }
}

impl TokenMethods for Rtr {
    fn new() -> Self {
        Rtr {
            times: 0,
        }
    }

    fn get_string_repr() -> String {
        "rtr".to_string()
    }

    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        if line[0] == "rtr" {
            return Ok(Rtr::params(line[1].parse().expect("Parsing from string to i32 failed")));
        }
        Err("Parsing Error".to_string())
    }
    fn parse(&self, input: &str) -> String {
        let mut s = input.to_string();

        for _ in 0..self.times {
            let c = s.remove(s.len() - 1).to_string();
            s = format!("{}{}", c, s);
        }

        s
    }

    fn token_to_atp_line(&self) -> String {
        format!("rtr {}", self.times)
    }
}
