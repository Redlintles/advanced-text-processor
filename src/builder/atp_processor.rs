use std::collections::HashMap;

use uuid::Uuid;

use crate::data::AtpToken;

use crate::parser::parser::parse_token;
use crate::parser::writer::write_to_file;

pub struct AtpProcessor {
    transforms: HashMap<String, Vec<AtpToken>>,
}

pub trait AtpProcessorMethods {
    fn write_to_file(&self, id: &str, path: &str) -> Result<(), String>;

    fn add_transform(&mut self, tokens: &Vec<AtpToken>) -> String;
    fn process_string(&self, id: &str, string: &str) -> String;
    // Step By Step
    fn process_sbs_string(&self, id: &str, string: &str) -> String;
}

impl AtpProcessor {
    pub fn new() -> Self {
        AtpProcessor { transforms: HashMap::new() }
    }
}

impl AtpProcessorMethods for AtpProcessor {
    fn write_to_file(&self, id: &str, path: &str) -> Result<(), String> {
        let tokens = self.transforms
            .get(id)
            .expect("Token array not found, is id a valid transform identifier");

        match write_to_file(path, tokens) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }

    fn process_string(&self, id: &str, string: &str) -> String {
        let mut result = String::from(string);

        let tokens = self.transforms
            .get(id)
            .expect("Token array not found, is id a valid transform identifier");

        for token in tokens.into_iter() {
            result = parse_token(token.clone(), result.as_str());
        }

        result.to_string()
    }

    fn process_sbs_string(&self, id: &str, string: &str) -> String {
        let mut result = String::from(string);

        let mut counter: i64 = 0;

        let tokens = self.transforms
            .get(id)
            .expect("Token array not found, is id a valid transform identifier");

        for token in tokens.into_iter() {
            let temp = parse_token(token.clone(), result.as_str());
            println!("[{}] => [{}]: {} => {}", counter, counter + 1, result, temp);

            result = String::from(temp);

            counter += 1;
        }

        result.to_string()
    }

    fn add_transform(&mut self, tokens: &Vec<AtpToken>) -> String {
        let identifier = Uuid::new_v4();
        self.transforms.insert(identifier.to_string(), tokens.clone());

        identifier.to_string()
    }
}
