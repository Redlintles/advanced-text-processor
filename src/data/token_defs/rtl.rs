use crate::data::{ TokenMethods };
#[derive(Clone)]
pub struct Rtl {
    pub times: usize,
}

impl Rtl {
    pub fn params(times: usize) -> Rtl {
        Rtl {
            times,
        }
    }

    pub fn new() -> Self {
        Rtl {
            times: 0,
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
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        if line[0] == "rtl" {
            self.times = line[1].parse().expect("Parsing from string to usize failed");
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "rtl".to_string()
    }
}
