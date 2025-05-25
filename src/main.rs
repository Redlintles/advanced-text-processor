pub mod builder;
pub mod data;
pub mod parser;

use std::{ fs::OpenOptions, io::{ BufRead, BufReader } };

use builder::{ atp_builder::Builder, atp_processor::AtpProcessorMethods };

fn main() {
    let (processor, default_identifer) = Builder::new()
        .delete_first()
        .delete_first()
        .delete_first()
        .replace_first_with(r";".to_string(), "".to_string())
        .replace_first_with(r"=".to_string(), "=>".to_string())
        .replace_first_with(r"\.".to_string(), "- ".to_string())
        .build();

    processor.write_to_file(&default_identifer, "example.atp");

    let file = OpenOptions::new().read(true).open("instructions.txt").unwrap();

    let reader = BufReader::new(file);

    for brute_line in reader.lines() {
        let line = brute_line.unwrap();
        println!("{}", processor.process_string(&default_identifer, &line));
    }
}
