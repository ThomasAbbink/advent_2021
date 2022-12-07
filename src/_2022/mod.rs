mod days;

use days::{day_1, day_2};

pub fn puzzles() -> Vec<fn()> {
    vec![day_1::run, day_2::run]
}
