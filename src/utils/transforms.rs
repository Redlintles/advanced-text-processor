pub fn string_to_usize(chunk: &String) -> Result<usize, String> {
    match chunk.parse() {
        Ok(v) => Ok(v),
        Err(_) => { Err("Parsing from string to usize failed".to_string()) }
    }
}
