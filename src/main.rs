mod days;
use std::env;

use days::{day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8};

fn main() {
    let puzzles = vec![
        day_1::run as fn(),
        day_2::run,
        day_3::run,
        day_4::run,
        day_5::run,
        day_6::run,
        day_7::run,
        day_8::run,
    ];
    let args: Vec<String> = env::args().collect();

    match args.len() {
        //run all puzzles
        1 => {
            for puzzle in puzzles {
                puzzle();
            }
        }
        2 => {
            // run a specific puzzle
            let puzzle_number = args[1].parse::<usize>().unwrap();
            puzzles[puzzle_number - 1]();
        }
        _ => println!("not running anything"),
    }
}
