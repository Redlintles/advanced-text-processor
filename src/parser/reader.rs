use std::{ fs::OpenOptions, io::{ BufRead, BufReader } };

use crate::data::{ Atb, Ate, AtpToken, Dla, Dlb, Dlc, Dlf, Dll, Raw, Rfw, Tbs, Tls, Trs };

fn parse_atb(tokens: Vec<String>) -> Atb {
    Atb { text: tokens[1].clone() }
}
fn parse_ate(tokens: Vec<String>) -> Ate {
    Ate { text: tokens[1].clone() }
}
fn parse_raw(tokens: Vec<String>) -> Raw {
    Raw { pattern: tokens[1].clone(), text_to_replace: tokens[2].clone() }
}
fn parse_rfw(tokens: Vec<String>) -> Rfw {
    Rfw { pattern: tokens[1].clone(), text_to_replace: tokens[2].clone() }
}
fn parse_dla(tokens: Vec<String>) -> Dla {
    Dla { index: tokens[1].clone().parse().expect("Parsing from string to usize failed!") }
}
fn parse_dlb(tokens: Vec<String>) -> Dlb {
    Dlb { index: tokens[1].clone().parse().expect("Parsing from string to usize failed!") }
}
fn parse_dlc(tokens: Vec<String>) -> Dlc {
    Dlc {
        start_index: tokens[1].clone().parse().expect("Parsing from string to usize failed!"),
        end_index: tokens[1].clone().parse().expect("Parsing from string to usize failed!"),
    }
}

pub fn read_from_file(path: &str) -> Vec<AtpToken> {
    let mut result: Vec<AtpToken> = Vec::new();

    let file = OpenOptions::new().read(true).open(path).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line_text = line.unwrap();

        line_text.pop();

        let chunks = shell_words::split(&line_text).unwrap();

        let token: AtpToken = match &chunks {
            x if x[0] == "atb" => AtpToken::Atb(parse_atb(chunks.clone())),
            x if x[0] == "ate" => AtpToken::Ate(parse_ate(chunks.clone())),
            x if x[0] == "raw" => AtpToken::Raw(parse_raw(chunks.clone())),
            x if x[0] == "rfw" => AtpToken::Rfw(parse_rfw(chunks.clone())),
            x if x[0] == "dla" => AtpToken::Dla(parse_dla(chunks.clone())),
            x if x[0] == "dlb" => AtpToken::Dlb(parse_dlb(chunks.clone())),
            x if x[0] == "dlc" => AtpToken::Dlc(parse_dlc(chunks.clone())),
            x if x[0] == "tbs" => AtpToken::Tbs(Tbs {}),
            x if x[0] == "trs" => AtpToken::Trs(Trs {}),
            x if x[0] == "tls" => AtpToken::Tls(Tls {}),
            x if x[0] == "dll" => AtpToken::Dll(Dll {}),
            x if x[0] == "dlf" => AtpToken::Dlf(Dlf {}),

            _ => panic!("Erro de parsing"),
        };

        result.push(token);
    }

    result
}
