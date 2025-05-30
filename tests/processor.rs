#[cfg(test)]
pub mod processor {
    use atp_project::builder::atp_processor::{ AtpProcessor, AtpProcessorMethods };

    #[test]
    fn test_read_from_file() {
        let mut processor = AtpProcessor::new();

        let identifier = processor.read_from_file("instructions.atp").unwrap();

        let input_string = "Banana";
        let expected_output = "Bznzn";

        let output = processor.process_sbs_string(&identifier, input_string);

        println!("{} => {} == {}", input_string, output, expected_output);

        assert_eq!(output, expected_output, "Unexpected Output in read_from_file");
    }
}
