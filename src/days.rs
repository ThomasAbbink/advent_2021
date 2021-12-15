use colored::Colorize;

pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

fn get_input(day_number: &i32) -> String {
    use std::fs;
    let path = format!("./src/input/day_{}.txt", day_number);
    fs::read_to_string(path).expect("unable to open file")
}

fn print_answer(day_number: &i32, task_number: &i32, result: &str) {
    let r = *day_number as u8 * (255 / 25) as u8;
    let g = 255 - *day_number as u8 * (255 / 25);
    let b = 255 - *day_number as u8 * (255 / 25);
    println!(
        "{} {}.{}: {}",
        "day".truecolor(r, g, b),
        day_number.to_string().truecolor(r, g, b),
        task_number.to_string().truecolor(r, g, b),
        result.truecolor(0, 204, 0).bold()
    );
}

#[macro_export]
macro_rules! solve {
    ($puzzle_number:expr, $parse:expr, $task_1:expr, $task_2:expr) => {{
        use crate::days::get_input;
        use crate::days::print_answer;
        let input = get_input($puzzle_number);
        let parsed = $parse(&input);
        let result_1 = $task_1(&parsed);
        print_answer($puzzle_number, &1, &result_1);
        let result_2 = $task_2(&parsed);
        print_answer($puzzle_number, &2, &result_2);
    }};
}
