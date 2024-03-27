use std::fs;

pub fn read_input_file(filepath: &str) -> String {
    fs::read_to_string(filepath).unwrap_or_else(|error| {
        panic!("{:?}", error);
    })
}
