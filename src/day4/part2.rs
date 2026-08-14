use std::{println, usize};
const EMPTY: &str = ".";

pub fn run() {
    let Ok(input) = super::super::input_loader::load_from_file("day4") else {
        panic!("load input failed for day4 step 2.")
    };
    println!("day4 step two - invalid ids sum : {}", solve_riddle(input));
}

pub fn solve_riddle(input: String) -> usize {
    let mut map: Vec<Vec<&str>> = vec![];
    let mut accessible_rolls = 0;
    input.split('\n').for_each(|rolls| {
        map.push(rolls.split("").filter(|c| *c != "").collect());
    });
    for line_index in 0..map.len() {
        accessible_rolls = accessible_rolls + count_accessible_rolls_on_line(line_index, &mut map);
        println!("{}", map[line_index].join(""));
    }

    accessible_rolls
}

fn count_accessible_rolls_on_line(line_index: usize, map: &mut Vec<Vec<&str>>) -> usize {
    let mut count = 0;
    if let Some(row) = map.get(line_index) {
        for index in 0..row.len() {
            count += if count_neighborhoud(line_index, index, map) < 4 {
                map[line_index][index] = "x";
                1
            } else {
                0
            }
        }
    }
    count
}

fn count_neighborhoud(line_index: usize, index: usize, map: &Vec<Vec<&str>>) -> usize {
    let mut count = 0;
    if map[line_index][index] == EMPTY {
        return 5;
    }
    if line_index >= 1 {
        if map[line_index - 1][index] != EMPTY {
            count += 1;
        }
        if index > 0 && map[line_index - 1][index - 1] != EMPTY {
            count += 1;
        }
        if index < map[line_index].len() - 1 && map[line_index - 1][index + 1] != EMPTY {
            count += 1;
        }
    }
    if line_index < map.len() - 1 {
        if map[line_index + 1][index] != EMPTY {
            count += 1;
        }
        if index > 0 && map[line_index + 1][index - 1] != EMPTY {
            count += 1;
        }
        if index < map[line_index].len() - 1 && map[line_index + 1][index + 1] != EMPTY {
            count += 1;
        }
    }
    if index > 0 && map[line_index][index - 1] != EMPTY {
        count += 1;
    }
    if index < map[line_index].len() - 1 && map[line_index][index + 1] != EMPTY {
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use crate::day4::part2::solve_riddle;

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
        assert_eq!(rolls_of_paper, 43);
    }
}
