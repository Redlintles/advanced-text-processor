use crate::{ data::AtpToken, parser::{ parse_token, writer::write_to_file } };

pub struct AtpProcessor {
    pub(crate) tokens: Vec<AtpToken>,
}

pub trait AtpProcessorMethods {
    fn write_to_file(&self, path: &str) -> ();
    fn process_string(&self, string: &str) -> String;
    // Step By Step
    fn process_sbs_string(&self, string: &str) -> String;
}

impl AtpProcessorMethods for AtpProcessor {
    fn write_to_file(&self, path: &str) -> () {
        write_to_file(path, &self.tokens);
    }

    fn process_string(&self, string: &str) -> String {
        let mut result = String::from(string);
        for token in self.tokens.iter() {
            result = parse_token(token.clone(), result.as_str());
        }

        result.to_string()
    }

    fn process_sbs_string(&self, string: &str) -> String {
        let mut result = String::from(string);

        let mut counter: i64 = 0;

        for token in self.tokens.iter() {
            let temp = parse_token(token.clone(), result.as_str());
            println!("[{}] => [{}]: {} => {}", counter, counter + 1, result, temp);

            result = String::from(temp);

            counter += 1;
        }

        result.to_string()
    }
}
