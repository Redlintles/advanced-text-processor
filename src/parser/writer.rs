use std::{ fs::OpenOptions, io::{ Write } };

use crate::data::{ TokenMethods };

pub fn write_to_file(path: &str, tokens: &Vec<Box<dyn TokenMethods>>) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|x| x.to_string())?;

    let mut success = true;

    for token in tokens.iter() {
        let line = token.token_to_atp_line();

        match file.write(line.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                success = false;
            }
        }
    }

    match success {
        true => Ok(()),
        false => Err("An unexpected error ocurred!".to_string()),
    }
}
