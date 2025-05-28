use crate::data::{ TokenFactory, TokenMethods };
#[derive(Clone)]
pub struct Rtr {
    pub times: usize,
}

impl Rtr {
    fn params(times: usize) -> Rtr {
        Rtr {
            times,
        }
    }
}

impl TokenMethods for Rtr {
    fn parse(&self, input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }

        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let times = self.times % len;

        chars[len - times..]
            .iter()
            .chain(&chars[..len - times])
            .collect()
    }

    fn token_to_atp_line(&self) -> String {
        format!("rtr {};\n", self.times)
    }
}

impl TokenFactory<Rtr> for Rtr {
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
            return Ok(Rtr::params(line[1].parse().expect("Parsing from string to usize failed")));
        }
        Err("Parsing Error".to_string())
    }
}
