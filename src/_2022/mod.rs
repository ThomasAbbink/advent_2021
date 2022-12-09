mod days;

use days::{day_1, day_2, day_3};

pub fn puzzles() -> Vec<fn()> {
    vec![day_1::run, day_2::run, day_3::run]
}
