use crate::data::TokenMethods;
#[derive(Clone)]
pub struct Rtl {
    pub times: i32,
}

impl Rtl {
    fn params(times: i32) -> Rtl {
        Rtl {
            times,
        }
    }
}

impl TokenMethods for Rtl {
    fn new() -> Self {
        Rtl {
            times: 0,
        }
    }

    fn get_string_repr() -> String {
        "rtl".to_string()
    }

    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String> {
        if line[0] == "rtl" {
            return Ok(Rtl::params(line[1].parse().expect("Parsing from string to i32 failed")));
        }
        Err("Parsing Error".to_string())
    }
    fn parse(&self, input: &str) -> String {
        let mut s = input.to_string();

        for _ in 0..self.times {
            let c = s.remove(0);
            s.push_str(&c.to_string());
        }

        s
    }

    fn token_to_atp_line(&self) -> String {
        format!("rtl {};\n", self.times)
    }
}
