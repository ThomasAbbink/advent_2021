use clap::Parser;

mod _2021;
mod _2022;
pub mod solve;

use _2021:: {
    puzzles as puzzles_2021
};
use _2022:: {
    puzzles as puzzles_2022
};

#[derive(Default, Parser, Debug)]
struct Arguments {
    #[clap(short)]
    year: Option<usize>,
    puzzle_number: Option<usize>,
}


fn get_puzzles(year:usize)-> Vec<fn()>{
    match year {
        2021 => puzzles_2021(),
        2022 => puzzles_2022(),
        _ => vec![]
    }
}

fn main() {
    let args = Arguments::parse();
    
    // default to the latest year
    let runnable: Vec<fn()> = match args.year {
        Some(year) =>  get_puzzles(year),
        None => get_puzzles(2022)
    };

    match args.puzzle_number {
        Some(number) => runnable[number -1](),
        None =>  for puzzle in runnable {
            puzzle();
        }  
    }
}
