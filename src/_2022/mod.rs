mod days;

use days::{
    day_1
};

pub fn puzzles() ->  Vec<fn()>{
    vec![
        day_1::run,
    ]
}