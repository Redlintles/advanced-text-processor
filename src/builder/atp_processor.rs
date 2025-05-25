use crate::data::AtpToken;

pub struct AtpProcessor {
    pub(crate) tokens: Vec<AtpToken>,
}

trait AtpProcessorMethods {
    fn write_to_file(path: &str) -> ();
    fn process_string(string: &str) -> String;
    // Step By Step
    fn process_sbs_string(string: &str) -> String;
}
