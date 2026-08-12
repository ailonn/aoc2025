pub fn run() {
    let Ok(input) = super::super::input_loader::load_from_file("day3") else {
        panic!("load input failed for day4 part1.")
    };
    println!("day4 step two - invalid ids sum : {}", solve_riddle(input));
}

fn solve_riddle(input: String) -> usize {
    input
        .split('\n')
        .map(|bank| {
            bank.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .map(|bank_batteries: Vec<usize>| todo!())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day4::part1::solve_riddle;

    #[test]
    fn input_from_instructions() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let rolls_of_paper = solve_riddle(input.into());
        assert_eq!(rolls_of_paper, 13);
    }

    #[test]
    fn instruction_line_by_line() {
        // assert_eq!(solve_riddle("987654321111111".into()), 98);
        // assert_eq!(solve_riddle("811111111111119".into()), 89);
        // assert_eq!(solve_riddle("234234234234278".into()), 78);
        // assert_eq!(solve_riddle("818181911112111".into()), 92);
    }
}
