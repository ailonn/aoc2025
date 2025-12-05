use std::fs;

fn load_input() -> Result<String, String> {
    let file_path = "src/day1/input.txt";
    println!("Input file {file_path}");

    if let Ok(contents) = fs::read_to_string(file_path) {
        Ok(contents)
    } else {
        return Err("Should have been able to read the file".into());
    }
}

fn day1() {
    let Ok(input) = load_input() else {
        panic!("load input failed for day 1.")
    };
    // let mut dial = 50;
    let vec_input: Vec<&str> = input.split('\n').collect();
    println!("rotations count {}", vec_input.len());
}

fn main() {
    day1();
}
