use std::fs;

pub fn load_from_file(day_folder_name: &str) -> Result<String, String> {
    let input_path = "src/".to_string() + day_folder_name + "/input.txt";
    println!("Input file {input_path}");

    if let Ok(contents) = fs::read_to_string(&input_path) {
        Ok(contents)
    } else {
        return Err("Should have been able to load input : ".to_owned() + &input_path);
    }
}
