pub fn run() {
    let Ok(input) = super::input_loader::load_from_file("src/day1/input.txt") else {
        panic!("load input failed for day 1.")
    };
    let password = give_me_door_password_from_instructions(input);
    println!("Password : {password}");
}

fn give_me_door_password_from_instructions(input: String) -> usize {
    let mut dial = 50;
    process_input(&input)
        .into_iter()
        .map(|(direction, count)| {
            let mut tick = 0;
            match direction {
                Direction::R => {
                    for _ in 1..=count {
                        if dial == 0 {
                            tick += 1;
                        }
                        if dial == 99 {
                            dial = 0;
                        } else {
                            dial += 1;
                        }
                    }
                },
                Direction::L => {
                    (1..=count).for_each(|_| {
                        if dial == 0 {
                            tick += 1;
                        }
                        if dial == 0 {
                            dial = 99;
                        } else {
                            dial -= 1;
                        }
                    });
                },
            };
            println!("The dial is rotated {direction:?}{count} to point at {dial}.");
            tick
        })
        .sum()
}

fn process_input(input: &str) -> Vec<(Direction, u32)> {
    input
        .split('\n')
        .map(|rotation: &str| {
            if rotation.starts_with('R') {
                return (Direction::R, rotation.trim_matches('R').parse().unwrap());
            }
            if rotation.starts_with('L') {
                return (Direction::L, rotation.trim_matches('L').parse().unwrap());
            }
            panic!("unknown rotation direction : {rotation}")
        })
        .collect()
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
    (1..=count).for_each(|_| {
        if dial == 0 {
            dial = 99;
        } else {
            dial -= 1;
        }
    });
    dial
}

#[derive(Debug, PartialEq)]
enum Direction {
    L,
    R,
}

#[cfg(test)]
mod tests {
    #[test]
    fn compute_password() {
        let input = "R3
L21
L32
R50
L50
R50
L1000";
        let password = crate::day1::give_me_door_password_from_instructions(input.to_string());
        assert_eq!(password, 12);
    }
}
