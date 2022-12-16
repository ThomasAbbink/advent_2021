mod days;

use days::{day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8};

pub fn puzzles() -> Vec<fn()> {
    vec![
        day_1::run,
        day_2::run,
        day_3::run,
        day_4::run,
        day_5::run,
        day_6::run,
        day_7::run,
        day_8::run,
    ]
}
