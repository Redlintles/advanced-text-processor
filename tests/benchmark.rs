#[cfg(test)]
pub mod benchmark {
    use std::time::Instant;
    use atp_project::builder::atp_builder::AtpBuilder; // Adjust the path if AtpBuilder is in a different crate or module
    use atp_project::builder::atp_processor::AtpProcessorMethods; // Adjust the path if AtpBuilder is in a different crate or module

    #[test]
    fn process_sbs_all_tokens() {
        let start = Instant::now();

        let string_to_process = "Banana Laranja cheia de canja";

        let (processor, identifier) = AtpBuilder::new()
            .add_to_beginning("Banana")
            .add_to_end("pizza")
            .repeat(3)
            .delete_after(20 as usize)
            .delete_before(2 as usize)
            .delete_chunk(0 as usize, 3 as usize)
            .delete_first()
            .delete_last()
            .replace_all_with(r"a", "e")
            .replace_first_with("L", "coxinha")
            .rotate_left(1 as usize)
            .rotate_right(2 as usize)
            .trim_both()
            .trim_left()
            .trim_right()
            .build();

        let processed_string = processor.process_sbs_string(&identifier, string_to_process);

        let _ = processor.write_to_file(&identifier, "Benchmark.atp");

        let elapsed = start.elapsed();

        println!(
            "A transformação de \"{}\" em \"{}\" levou: {:.6} Segundos",
            string_to_process,
            processed_string,
            elapsed.as_secs_f64()
        );

        assert!(elapsed.as_secs_f64() < 0.003, "Executou muito devagar");
    }
}
