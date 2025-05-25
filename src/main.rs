pub mod builder;
pub mod data;
pub mod parser;

use builder::{ atp_builder::Builder, atp_processor::AtpProcessorMethods };

fn main() {
    let (processor, default_identifer) = Builder::new()
        .add_to_beginning(String::from("Banana"))
        .replace_all_with(String::from(r"a"), String::from("z"))
        .delete_after(5 as usize)
        .trim_both()
        .delete_chunk(1 as usize, 3 as usize)
        .build();

    processor.write_to_file(&default_identifer, "example.atp");

    println!("{}", processor.process_sbs_string(&default_identifer, "Caxinhos dourados"));
}
