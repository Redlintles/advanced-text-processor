use std::{ fs::OpenOptions, io::{ BufRead, BufReader } };

use crate::data::{ TokenMethods };

use super::get_supported_tokens;

pub fn read_from_file(path: &str) -> Result<Vec<Box<dyn TokenMethods>>, String> {
    let mut result: Vec<Box<dyn TokenMethods>> = Vec::new();

    let file = match OpenOptions::new().read(true).open(path) {
        Ok(x) => x,
        Err(_) => {
            return Err("An ATP parsing error ocurred: File not found".to_string());
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line_text = match line {
            Ok(x) => x,
            Err(_) => {
                return Err("An ATP Parsing error ocurred: Error reading file line".to_string());
            }
        };

        line_text.pop();

        let chunks = match shell_words::split(&line_text) {
            Ok(x) => x,
            Err(_) => {
                return Err("An ATP parsing error ocurred: Shell Words internal error".to_string());
            }
        };

        let supported_tokens = get_supported_tokens();
        let token_factory = match supported_tokens.get(&chunks[0]) {
            Some(x) => x,
            None => {
                return Err("An ATP parsing error ocurred: Invalid Sintax".to_string());
            }
        };

        let mut token = token_factory();

        match token.token_from_vec_params(chunks) {
            Ok(x) => x,
            Err(_) => {
                return Err("An Atp parsing error ocurred: Invalid token".to_string());
            }
        }

        result.push(token);
    }

    Ok(result)
}
