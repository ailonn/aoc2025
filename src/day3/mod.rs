pub fn run() {
    let Ok(input) = super::input_loader::load_from_file("day3") else {
        panic!("load input failed for day3.")
    };
    println!("day3 step one - invalid ids sum : {}", solve_riddle(input));
}

fn solve_riddle(input: String) -> usize {
    input
        .split('\n')
        .map(|bank| {
            bank.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .map(|bank_batteries: Vec<usize>| {
            let max = bank_batteries.iter().max().unwrap();
            let mut max_indice: usize = bank_batteries.len();
            for index in 0..bank_batteries.len() {
                let current_battery = bank_batteries.get(index).unwrap();
                if current_battery == max {
                    max_indice = index;
                    break;
                }
            }
            if max_indice == bank_batteries.len() {
                panic!("not found {max} indice");
            }
            if max_indice + 1 == bank_batteries.len()
                && let Some((_last, elements)) = bank_batteries.split_last()
            {
                return elements.iter().max().unwrap() * 10 + max;
            }
            let (_left, elements) = bank_batteries.split_at(max_indice + 1);
            return max * 10 + elements.iter().max().unwrap();
        })
        .sum()
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
        assert_eq!(joltage, 3121910778619);
    }

    #[test]
    fn simple_input() {
        let input = "12345";
        let joltage = solve_riddle(input.into());
        assert_eq!(joltage, 12345);
    }

    #[test]
    fn input_of_one_bank() {
        let input = "818181911112111";
        let joltage = solve_riddle(input.into());
        assert_eq!(joltage, 888911112111);
    }
}
