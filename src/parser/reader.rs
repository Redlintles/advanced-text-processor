use std::{ fs::OpenOptions, io::{ BufRead, BufReader } };

use crate::data::token_defs::*;

use crate::data::{ AtpToken, TokenMethods };

pub fn read_from_file(path: &str) -> Vec<AtpToken> {
    let mut result: Vec<AtpToken> = Vec::new();

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

        let token: AtpToken = match &chunks {
            x if x[0] == "atb" =>
                AtpToken::Atb(atb::Atb::token_from_vec_params(chunks).expect(parse_err_msg)),
            x if x[0] == "ate" =>
                AtpToken::Ate(ate::Ate::token_from_vec_params(chunks).expect(parse_err_msg)),
            x if x[0] == "raw" =>
                AtpToken::Raw(raw::Raw::token_from_vec_params(chunks).expect(parse_err_msg)),
            x if x[0] == "rfw" =>
                AtpToken::Rfw(rfw::Rfw::token_from_vec_params(chunks).expect(parse_err_msg)),
            x if x[0] == "dla" =>
                AtpToken::Dla(dla::Dla::token_from_vec_params(chunks).expect(parse_err_msg)),
            x if x[0] == "dlb" =>
                AtpToken::Dlb(dlb::Dlb::token_from_vec_params(chunks).expect(parse_err_msg)),
            x if x[0] == "dlc" =>
                AtpToken::Dlc(dlc::Dlc::token_from_vec_params(chunks).expect(parse_err_msg)),
            x if x[0] == "rtl" =>
                AtpToken::Rtl(rtl::Rtl::token_from_vec_params(chunks).expect(parse_err_msg)),
            x if x[0] == "tbs" => AtpToken::Tbs(tbs::Tbs::new()),
            x if x[0] == "trs" => AtpToken::Trs(trs::Trs::new()),
            x if x[0] == "tls" => AtpToken::Tls(tls::Tls::new()),
            x if x[0] == "dll" => AtpToken::Dll(dll::Dll::new()),
            x if x[0] == "dlf" => AtpToken::Dlf(dlf::Dlf::new()),

            _ => panic!("An ATP parsing error ocurred: Token not found"),
        };

        result.push(token);
    }

    result
}
