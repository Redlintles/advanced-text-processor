use crate::data::{ TokenFactory, TokenMethods };
#[derive(Clone)]
pub struct Rtl {
    pub times: usize,
}

impl Rtl {
    fn params(times: usize) -> Rtl {
        Rtl {
            times,
        }
    }
}

impl TokenMethods for Rtl {
    fn parse(&self, input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }

        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let times = self.times % len;

        chars[times..]
            .iter()
            .chain(&chars[..times])
            .collect()
    }

    fn token_to_atp_line(&self) -> String {
        format!("rtl {};\n", self.times)
    }
}

impl TokenFactory<Rtl> for Rtl {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        if line[0] == "rtl" {
            return Ok(Rtl::params(line[1].parse().expect("Parsing from string to usize failed")));
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr() -> String {
        "rtl".to_string()
    }
    fn new() -> Self {
        Rtl {
            times: 0,
        }
    }
}
