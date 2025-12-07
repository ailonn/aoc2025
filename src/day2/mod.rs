pub fn run() {
    let Ok(input) = super::input_loader::load_from_file("day2") else {
        panic!("load input failed for day 2.")
    };
    println!("day2 step one - invalid ids sum : {}", solve_riddle(input));
}

fn solve_riddle(input: String) -> usize {
    input
        .split(",")
        .map(|interval| interval.split('-').collect())
        .map(|interval_bound: Vec<&str>| -> usize {
            let Ok(start) = interval_bound.get(0).unwrap().parse::<usize>() else {
                panic!("paseint error on interval_bound.get(0)")
            };
            let Ok(end) = interval_bound.get(1).unwrap().parse::<usize>() else {
                panic!("paseint error on interval_bound.get(1)")
            };
            (start..=end).filter(is_an_invalid_id).sum()
        })
        .sum()
}

fn is_an_invalid_id(id: &usize) -> bool {
    let str_id = id.to_string();
    if str_id.len() % 2 != 0 {
        return false;
    }
    let (part1, part2) = str_id.split_at(str_id.len() / 2);
    part1 == part2
}

#[cfg(test)]
mod test {
    use crate::day2::solve_riddle;

    #[test]
    fn input_from_instructions() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let sum = solve_riddle(input.into());
        assert_eq!(sum, 4174379265);
    }

    #[test]
    fn input_of_one_interval() {
        let input = "11-22";
        let sum = solve_riddle(input.into());
        assert_eq!(sum, 33);
    }

    #[test]
    fn input_of_three_intervals() {
        let input = "11-22,95-115,998-1012";
        let sum = solve_riddle(input.into());
        assert_eq!(sum, 2252);
    }
}
