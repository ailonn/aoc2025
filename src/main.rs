mod day1;
mod day2;
mod day3;
mod input_loader;

use std::env;

const DAYS_SOLVER: [fn(); 3] = [day1::run, day2::run, day3::run];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("resolve all implemented day");
        DAYS_SOLVER.iter().for_each(|d| {
            d();
        });
    } else {
        let Ok(day) = &args[1].parse::<usize>() else {
            panic!("a number is expected instead of : {}", &args[1]);
        };
        let Some(solver) = DAYS_SOLVER.get(*day - 1) else {
            panic!("day {day} is not implemented");
        };
        solver();
        println!("resolved day {day}");
    }
}
