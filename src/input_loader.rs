use std::fs;

pub fn load_from_file(file_path: &str) -> Result<String, String> {
    println!("Input file {file_path}");

    if let Ok(contents) = fs::read_to_string(file_path) {
        Ok(contents)
    } else {
        return Err("Should have been able to read the file".into());
    }
}
