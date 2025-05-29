use std::{ fs::OpenOptions, io::{ BufRead, BufReader } };

use crate::data::{ TokenMethods };

use super::get_supported_tokens;

pub fn read_from_file(path: &str) -> Vec<Box<dyn TokenMethods>> {
    let mut result: Vec<Box<dyn TokenMethods>> = Vec::new();

    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("An ATP parsing error ocurred: File not found");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line_text = line.expect("An atp Parsing error ocurred: Error reading file line");

        line_text.pop();

        let chunks = shell_words
            ::split(&line_text)
            .expect("An ATP parsing error ocurred: Shell Words internal error");

        let parse_err_msg = "An ATP parsing error ocurred: Invalid Sintax";

        let supported_tokens = get_supported_tokens();
        let token_factory = supported_tokens.get(&chunks[0]).expect(parse_err_msg);

        let mut token = token_factory();

        token.token_from_vec_params(chunks);

        result.push(token);
    }

    result
}
