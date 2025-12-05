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

fn increment_dial(mut dial: u32, count: u32) -> u32 {
    for _ in 1..=count {
        if dial == 99 {
            dial = 0;
        } else {
            dial += 1;
        }
    }
    dial
}
fn decrement_dial(mut dial: u32, count: u32) -> u32 {
    for _ in 1..=count {
        if dial == 0 {
            dial = 99;
        } else {
            dial -= 1;
        }
    }
    dial
}

fn day1() {
    #[derive(Debug)]
    enum Direction {
        L,
        R,
    }

    let Ok(input) = load_input() else {
        panic!("load input failed for day 1.")
    };
    let mut dial = 50;
    let mut password = 0;
    input
        .split('\n')
        .map(|rotation: &str| {
            if rotation.starts_with('R') {
                return (Direction::R, rotation.trim_matches('R').parse().unwrap());
            }
            if rotation.starts_with('L') {
                return (Direction::L, rotation.trim_matches('L').parse().unwrap());
            }
            panic!("missing direction in day 1 input");
        })
        .for_each(|(direction, count)| {
            match direction {
                Direction::R => {
                    dial = increment_dial(dial, count);
                }
                Direction::L => {
                    dial = decrement_dial(dial, count);
                }
            }
            if dial == 0 {
                password += 1;
            }
            println!("The dial is rotated {direction:?}{count} to point at {dial}.");
            println!("Password : {password}");
        });
}

fn main() {
    day1();
}
