use std::{ fs::OpenOptions, io::Write };

use crate::data::{ AtpToken, TokenMethods };

pub fn write_to_file(path: &str, tokens: &Vec<AtpToken>) {
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path).unwrap();

    for token in tokens.iter() {
        let line = match token {
            AtpToken::Atb(obj) => obj.token_to_atp_line(),
            AtpToken::Ate(obj) => obj.token_to_atp_line(),
            AtpToken::Dla(obj) => obj.token_to_atp_line(),
            AtpToken::Dlb(obj) => obj.token_to_atp_line(),
            AtpToken::Dlc(obj) => obj.token_to_atp_line(),
            AtpToken::Dlf(obj) => obj.token_to_atp_line(),
            AtpToken::Dll(obj) => obj.token_to_atp_line(),
            AtpToken::Raw(obj) => obj.token_to_atp_line(),
            AtpToken::Rfw(obj) => obj.token_to_atp_line(),
            AtpToken::Tbs(obj) => obj.token_to_atp_line(),
            AtpToken::Tls(obj) => obj.token_to_atp_line(),
            AtpToken::Trs(obj) => obj.token_to_atp_line(),
        };

        match file.write(line.as_bytes()) {
            Ok(_) => (),
            Err(_) => panic!("Erro ao escrever no arquivo"),
        }
    }
}
