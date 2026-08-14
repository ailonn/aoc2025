use std::{println, usize};

pub fn run() {
    let Ok(input) = super::super::input_loader::load_from_file("day3") else {
        panic!("load input failed for day3 step 2.")
    };
    println!("day3 step two - invalid ids sum : {}", solve_riddle(input));
}

fn solve_riddle(input: String) -> usize {
    input
        .split('\n')
        .map(|bank| {
            bank.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .map(|bank_batteries: Vec<usize>| turn_on_battery_to_get_max_voltage(&bank_batteries))
        .sum()
}

fn turn_on_battery_to_get_max_voltage(bank_batteries: &Vec<usize>) -> usize {
    let mut battery_activated: Vec<usize> = bank_batteries.clone();
    battery_activated.fill(0);
    let mut bank: Vec<usize> = bank_batteries.clone();
    const BANK_TO_TURN_ON: usize = 12;

    let mut last_battery_activated_index = 0;
    let mut count_activated_batteries = 0;
    while count_activated_batteries < BANK_TO_TURN_ON.into() {
        let left_to_found = BANK_TO_TURN_ON - count_activated_batteries;
        let bank_right_bound = bank.len() - left_to_found;
        let bank_left_bound = if last_battery_activated_index == 0 {
            0
        } else {
            last_battery_activated_index + 1
        };

        let (max, relative_max_indice) =
            found_max_and_max_index_on_sub_array(&bank[bank_left_bound..=(bank_right_bound)]);

        let max_indice = relative_max_indice + bank_left_bound;
        bank[max_indice] = 0;
        battery_activated[max_indice] = max;
        last_battery_activated_index = max_indice;
        count_activated_batteries = battery_activated
            .clone()
            .into_iter()
            .filter(|v| *v != 0)
            .count();
    }

    let parse = battery_activated
        .extract_if(.., |x| *x != 0)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    parse
}

fn found_max_and_max_index_on_sub_array(arr: &[usize]) -> (usize, usize) {
    let mut index: usize = 0;
    let mut max: usize = 0;
    let mut max_index: usize = 0;
    while index < arr.len() {
        if arr[index] > max {
            max = arr[index];
            max_index = index;
        }

        index = index + 1;
    }
    (max, max_index)
}

#[cfg(test)]
mod test {
    use crate::day3::part2::solve_riddle;

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
    fn instruction_line_by_line() {
        assert_eq!(solve_riddle("987654321111111".into()), 987654321111);
        assert_eq!(solve_riddle("811111111111119".into()), 811111111119);
        assert_eq!(solve_riddle("234234234234278".into()), 434234234278);
        assert_eq!(solve_riddle("818181911112111".into()), 888911112111);
    }
}
