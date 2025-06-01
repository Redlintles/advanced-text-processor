use std::collections::HashMap;
use std::path::Path;

use uuid::Uuid;
use colored::*;

use crate::data::{ TokenMethods };

use crate::text_parser::parser::parse_token;
use crate::text_parser::reader::read_from_file;
use crate::text_parser::writer::write_to_file;
#[derive(Default)]
pub struct AtpProcessor {
    transforms: HashMap<String, Vec<Box<dyn TokenMethods>>>,
}

pub trait AtpProcessorMethods {
    fn write_to_file(&self, id: &str, path: &str) -> Result<(), String>;
    fn read_from_file(&mut self, path: &str) -> Result<String, String>;
    fn add_transform(&mut self, tokens: Vec<Box<dyn TokenMethods>>) -> String;
    fn process_all(&self, id: &str, input: &str) -> String;
    fn process_all_with_debug(&self, id: &str, input: &str) -> String;
    fn process_single(&self, token: Box<dyn TokenMethods>, input: &str) -> String;
    fn process_single_with_debug(&self, token: Box<dyn TokenMethods>, input: &str) -> String;
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

        match write_to_file(Path::new(path), tokens) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }

    fn read_from_file(&mut self, path: &str) -> Result<String, String> {
        match read_from_file(Path::new(path)) {
            Ok(tokens) => {
                let identifier = Uuid::new_v4();

                self.transforms.insert(identifier.to_string(), tokens);

                Ok(identifier.to_string())
            }
            Err(e) => Err(e),
        }
    }

    fn process_all(&self, id: &str, input: &str) -> String {
        let mut result = String::from(input);

        let tokens = self.transforms
            .get(id)
            .expect("Token array not found, is id a valid transform identifier");

        for token in tokens.iter() {
            result = parse_token(token.as_ref(), result.as_str());
        }

        result.to_string()
    }

    fn process_all_with_debug(&self, id: &str, input: &str) -> String {
        let mut result = String::from(input);

        let dashes = 10;

        let tokens = self.transforms
            .get(id)
            .expect("Token array not found, is id a valid transform identifier");

        println!("PROCESSING STEP BY STEP:\n{}\n", "-".repeat(dashes));

        for (counter, token) in (0_i64..).zip(tokens.iter()) {
            let temp = parse_token(token.as_ref(), result.as_str());
            println!(
                "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
                counter.to_string().blue(),
                (counter + 1).to_string().blue(),
                token.token_to_atp_line().yellow(),
                result.red(),
                temp.green()
            );

            if (counter as usize) < tokens.len() {
                println!("{}\n", "-".repeat(dashes));
            }

            result = temp;
        }

        result.to_string()
    }

    fn add_transform(&mut self, tokens: Vec<Box<dyn TokenMethods>>) -> String {
        let identifier = Uuid::new_v4();
        self.transforms.insert(identifier.to_string(), tokens.clone());

        identifier.to_string()
    }

    fn process_single(&self, token: Box<dyn TokenMethods>, input: &str) -> String {
        token.parse(input)
    }

    fn process_single_with_debug(&self, token: Box<dyn TokenMethods>, input: &str) -> String {
        let output = token.parse(input);
        println!(
            "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
            (0).to_string().blue(),
            (1).to_string().blue(),
            token.token_to_atp_line().yellow(),
            input.red(),
            output.green()
        );

        output
    }
}
