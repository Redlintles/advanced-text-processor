use std::collections::HashMap;

use uuid::Uuid;

use crate::{ data::AtpToken, parser::{ parse_token, writer::write_to_file } };

pub struct AtpProcessor {
    transforms: HashMap<String, Vec<AtpToken>>,
}

pub trait AtpProcessorMethods {
    fn write_to_file(&self, id: &String, path: &str) -> ();

    fn add_transform(&mut self, tokens: &Vec<AtpToken>) -> String;
    fn process_string(&self, id: &String, string: &str) -> String;
    // Step By Step
    fn process_sbs_string(&self, id: &String, string: &str) -> String;
}

impl AtpProcessor {
    pub fn new() -> Self {
        AtpProcessor { transforms: HashMap::new() }
    }
}

impl AtpProcessorMethods for AtpProcessor {
    fn write_to_file(&self, id: &String, path: &str) -> () {
        write_to_file(path, &self.transforms.get(id).unwrap());
    }

    fn process_string(&self, id: &String, string: &str) -> String {
        let mut result = String::from(string);

        let tokens = self.transforms.get(id).unwrap();

        for token in tokens.into_iter() {
            result = parse_token(token.clone(), result.as_str());
        }

        result.to_string()
    }

    fn process_sbs_string(&self, id: &String, string: &str) -> String {
        let mut result = String::from(string);

        let mut counter: i64 = 0;

        let tokens = self.transforms.get(id).unwrap();

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
