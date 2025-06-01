use std::{ fs::OpenOptions, io::Write, path::Path };

use crate::{ data::TokenMethods, parser::reader::read_from_file };

use super::{ reader::read_bytecode_from_file, token_to_bytecode_token_convert };

pub fn atp_text_to_bytecode_file(input_file: &Path, output_file: &Path) -> Result<(), String> {
    if input_file.extension().unwrap() != "atp" {
        return Err(
            "An Atp Transforming error ocurred: Only atp file extensions are allowed".to_string()
        );
    }
    if output_file.extension().unwrap() != "atpbc" {
        return Err(
            "An Atp Transforming error ocurred: Only atpbc file extensions are allowed".to_string()
        );
    }

    let tokens: Vec<Box<dyn TokenMethods>> = read_from_file(
        input_file.to_str().expect("Invalid path")
    )?;

    let mut new_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_file)
        .map_err(|x| x.to_string())?;

    for token in tokens.into_iter() {
        let line = token_to_bytecode_token_convert(token)
            .expect("Invalid Path")
            .token_to_bytecode_instruction()
            .to_bytecode_line();

        match new_file.write(line.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
    Ok(())
}

pub fn atp_bytecode_to_atp_file(input_file: &Path, output_file: &Path) -> Result<(), String> {
    if input_file.extension().unwrap() != "atpbc" {
        return Err(
            "An Atp Transforming error ocurred: Only atpbc file extensions are allowed".to_string()
        );
    }
    if output_file.extension().unwrap() != "atp" {
        return Err(
            "An Atp Transforming error ocurred: Only atp file extensions are allowed".to_string()
        );
    }
    let tokens = read_bytecode_from_file(input_file)?;

    let mut new_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_file)
        .map_err(|e| e.to_string())?;

    for token in tokens.into_iter() {
        let line = token.token_to_atp_line();

        match new_file.write(line.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    Ok(())
}
