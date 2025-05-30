use std::collections::HashMap;

use uuid::Uuid;

use crate::data::{ TokenMethods };

use crate::parser::parser::parse_token;
use crate::parser::reader::read_from_file;
use crate::parser::writer::write_to_file;

pub struct AtpProcessor {
    transforms: HashMap<String, Vec<Box<dyn TokenMethods>>>,
}

pub trait AtpProcessorMethods {
    fn write_to_file(&self, id: &str, path: &str) -> Result<(), String>;
    fn read_from_file(&mut self, path: &str) -> Result<String, String>;
    fn add_transform(&mut self, tokens: Vec<Box<dyn TokenMethods>>) -> String;
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

    fn read_from_file(&mut self, path: &str) -> Result<String, String> {
        match read_from_file(path) {
            Ok(tokens) => {
                let identifier = Uuid::new_v4();

                self.transforms.insert(identifier.to_string(), tokens);

                return Ok(identifier.to_string());
            }
            Err(e) => Err(e),
        }
    }

    fn process_string(&self, id: &str, string: &str) -> String {
        let mut result = String::from(string);

        let tokens = self.transforms
            .get(id)
            .expect("Token array not found, is id a valid transform identifier");

        for token in tokens.iter() {
            result = parse_token(token, result.as_str());
        }

        result.to_string()
    }

    fn process_sbs_string(&self, id: &str, string: &str) -> String {
        let mut result = String::from(string);

        let mut counter: i64 = 0;

        let tokens = self.transforms
            .get(id)
            .expect("Token array not found, is id a valid transform identifier");

        for token in tokens.iter() {
            let temp = parse_token(token, result.as_str());
            println!(
                "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
                counter,
                counter + 1,
                token.token_to_atp_line(),
                result,
                temp
            );

            result = String::from(temp);

            counter += 1;
        }

        result.to_string()
    }

    fn add_transform(&mut self, tokens: Vec<Box<dyn TokenMethods>>) -> String {
        let identifier = Uuid::new_v4();
        self.transforms.insert(identifier.to_string(), tokens.clone());

        identifier.to_string()
    }
}
