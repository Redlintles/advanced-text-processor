use std::{ fs::OpenOptions, io::Write, path::Path };

use super::BytecodeTokenMethods;

pub fn write_bytecode_to_file(
    path: &Path,
    tokens: Vec<Box<dyn BytecodeTokenMethods>>
) -> Result<(), String> {
    if path.extension() != "atpbc" {
        return Err("An ATPBC writing error ocurred: new file must have .atpbc Extensions");
    }
    let mut file = match OpenOptions::new().create(true).truncate(true).write(true).open(path) {
        Ok(x) => x,
        Err(_) => {
            return Err("An Atp Bytecode Error Ocurred: Cannot open file to write".to_string());
        }
    };

    for token in tokens.iter() {
        let line = token.token_to_bytecode_instruction().to_bytecode_line();

        match file.write(line.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                return Err("An Atp Bytecode Error Ocurred: Cannot write on file".to_string());
            }
        }
    }

    Ok(())
}
