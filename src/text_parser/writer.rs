use std::{ fs::OpenOptions, io::Write, path::Path };

use crate::token_data::{ TokenMethods };

pub fn write_to_file(path: &Path, tokens: &Vec<Box<dyn TokenMethods>>) -> Result<(), String> {
    if path.extension().expect("Unable to get file extension") != "atp" {
        return Err(
            "An ATP Writing error ocurred: Only .atp files are allowed to be written".to_string()
        );
    }
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|x| x.to_string())?;

    let mut success = true;

    for token in tokens.iter() {
        let line = token.token_to_atp_line();

        match file.write_all(line.as_bytes()) {
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
