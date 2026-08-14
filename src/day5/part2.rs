pub fn run() {
    let Ok(input) = super::super::input_loader::load_from_file("day5") else {
        panic!("load input failed for day5 part 2.")
    };
    println!(
        "day5 step two - count of fresh ingredient : {}",
        solve_riddle(input)
    );
}

pub fn solve_riddle(input: String) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::day5::part1::solve_riddle;

    #[test]
    fn input_from_instructions() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        // let rolls_of_paper = solve_riddle(input.into());
        // assert_eq!(rolls_of_paper, 3);
    }
}
