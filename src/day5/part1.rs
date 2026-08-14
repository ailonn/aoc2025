pub fn run() {
    let Ok(input) = super::super::input_loader::load_from_file("day5") else {
        panic!("load input failed for day5 part 1.")
    };
    println!(
        "day5 step one - count of fresh ingredient : {}",
        solve_riddle(input)
    );
}

pub fn solve_riddle(input: String) -> usize {
    let mut ranges = vec![];
    let mut ids = vec![];
    input.split('\n').for_each(|line| {
        if line.contains("-") {
            ranges.push(line);
        } else if line != "" {
            ids.push(line)
        }
    });
    ids.iter()
        .map(|id| is_fresh_id(id, &ranges))
        .filter(|is_fresh| *is_fresh)
        .count()
}

fn is_fresh_id(id: &str, ranges: &[&str]) -> bool {
    for range in ranges {
        let parsed_range_edge: Result<Vec<usize>, _> =
            range.split("-").map(|edge| edge.parse::<usize>()).collect();
        match parsed_range_edge {
            Ok(range_edge) => {
                assert_eq!(range_edge.len(), 2);
                let Some(min) = range_edge.get(0) else {
                    panic!();
                };
                let Some(max) = range_edge.get(1) else {
                    panic!();
                };
                match id.parse::<usize>() {
                    Ok(usize_id) if usize_id >= *min && usize_id <= *max => return true,
                    Err(err) => panic!("{err}, {id}, {min}, {max}"),
                    Ok(_) => (),
                }
            }
            Err(_) => todo!(),
        }
    }
    false
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
        let rolls_of_paper = solve_riddle(input.into());
        assert_eq!(rolls_of_paper, 3);
    }
}
