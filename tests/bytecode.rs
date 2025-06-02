#[cfg(test)]
#[cfg(feature = "test_access")]
pub mod bytecode {
    use std::{ fs::File, io::Read, path::Path };

    use atp_project::builder::atp_processor::AtpProcessorDebugMethods;

    #[test]
    fn test_write_bytecode_to_file() {
        use atp_project::bytecode_parser::{ writer::write_bytecode_to_file, BytecodeTokenMethods };
        use atp_project::token_data::token_defs::{ atb::Atb, rpt::Rpt, ate::Ate };
        let file = tempfile::NamedTempFile::new().expect("Error opening archive");

        let path = file.path();

        let tokens: Vec<Box<dyn BytecodeTokenMethods>> = vec![
            Box::new(Atb {
                text: "Banana".to_string(),
            }),
            Box::new(Ate {
                text: "Laranja".to_string(),
            }),
            Box::new(Rpt {
                times: 3 as usize,
            })
        ];

        let _ = write_bytecode_to_file(path, tokens);

        let mut opened_file = File::open(path).unwrap();

        let mut content = String::new();
        opened_file.read_to_string(&mut content).unwrap();

        let expected_content = "0x01 Banana\n0x02 Laranja\n0x0d 3\n";

        assert_eq!(
            content,
            expected_content,
            "Unexpected Output in test_write_to_file: content differs"
        );
    }

    #[test]
    fn test_read_bytecode_from_file() {
        use atp_project::{
            builder::atp_processor::{ AtpProcessor },
            bytecode_parser::reader::read_bytecode_from_file,
            token_data::TokenMethods,
        };
        let result = match read_bytecode_from_file(Path::new("banana.atpbc")) {
            Ok(x) => x,
            Err(_) => panic!("Erro de leitura"),
        };

        let input = "Coxinha";

        let expected_output = "BananaCoxinhaLaranjaBananaCoxinhaLaranjaBananaCoxinhaLaranja";

        let mut processor: Box<dyn AtpProcessorDebugMethods> = Box::new(AtpProcessor::new());

        let tokens: Vec<Box<dyn TokenMethods>> = result
            .into_iter()
            .map(|token| token as Box<dyn TokenMethods>)
            .collect();

        let identifier = processor.add_transform(tokens);

        let output = processor.process_all_with_debug(&identifier, input).unwrap();

        assert_eq!(output, expected_output);
    }
}
