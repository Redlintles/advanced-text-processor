use std::{ fs::OpenOptions, io::Write };

use crate::data::AtpToken;

pub fn write_to_file(path: &str, tokens: &Vec<AtpToken>) {
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path).unwrap();

    for token in tokens.iter() {
        let line = match token {
            AtpToken::Atb(obj) => format!("atb \"{}\";\n", obj.text),
            AtpToken::Ate(obj) => format!("ate \"{}\";\n", obj.text),
            AtpToken::Dla(obj) => format!("dla {};\n", obj.index),
            AtpToken::Dlb(obj) => format!("dlb {};\n", obj.index),
            AtpToken::Dlc(obj) => format!("dlc {} {};\n", obj.start_index, obj.end_index),
            AtpToken::Dlf(_) => format!("dlf;\n"),
            AtpToken::Dll(_) => format!("dll;\n"),
            AtpToken::Raw(obj) =>
                format!("raw r\"{}\" \"{}\";\n", obj.pattern, obj.text_to_replace),
            AtpToken::Rfw(obj) =>
                format!("rfw r\"{}\" \"{}\";\n", obj.pattern, obj.text_to_replace),
            AtpToken::Tbs(_) => format!("tbs;\n"),
            AtpToken::Tls(_) => format!("tls;\n"),
            AtpToken::Trs(_) => format!("trs;\n"),
        };

        match file.write(line.as_bytes()) {
            Ok(_) => (),
            Err(_) => panic!("Erro ao escrever no arquivo"),
        }
    }
}
