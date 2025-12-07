pub fn run() {
    let Ok(input) = super::input_loader::load_from_file("day3") else {
        panic!("load input failed for day3.")
    };
    println!("day3 step one - invalid ids sum : {}", solve_riddle(input));
}

fn solve_riddle(input: String) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::day3::solve_riddle;

    #[test]
    fn input_from_instructions() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let joltage = solve_riddle(input.into());
        assert_eq!(joltage, 357);
    }

    #[test]
    fn simple_input() {
        let input = "12345";
        let joltage = solve_riddle(input.into());
        assert_eq!(joltage, 24);
    }

    #[test]
    fn input_of_one_bank() {
        let input = "818181911112111";
        let joltage = solve_riddle(input.into());
        assert_eq!(joltage, 92);
    }
}
