use std::{println, usize};

pub fn run() {
    let Ok(input) = super::super::input_loader::load_from_file("day4") else {
        panic!("load input failed for day4 step 2.")
    };
    println!("day4 step two - invalid ids sum : {}", solve_riddle(input));
}

pub fn solve_riddle(input: String) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::day4::part2::solve_riddle;

    #[test]
    fn input_from_instructions() {
        //         let input = "..@@.@@@@.
        // @@@.@.@.@@
        // @@@@@.@.@@
        // @.@@@@..@.
        // @@.@@@@.@@
        // .@@@@@@@.@
        // .@.@.@.@@@
        // @.@@@.@@@@
        // .@@@@@@@@.
        // @.@.@@@.@.";
        //         let rolls_of_paper = solve_riddle(input.into());
        //         assert_eq!(rolls_of_paper, 13);
    }
}
