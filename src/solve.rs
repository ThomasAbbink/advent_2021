use colored::Colorize;

pub fn get_input(year_number: &i32, day_number: &i32) -> String {
    use std::fs;
    let path = format!("./src//_{}/input/day_{}.txt", year_number, day_number);
    dbg!(&path);
    fs::read_to_string(path).expect("unable to open the input file")
}

pub fn print_answer(day_number: &i32, task_number: &i32, result: &str) {
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
    ($year_number:expr, $puzzle_number:expr, $parse:expr, $task_1:expr, $task_2:expr) => {{
        use crate::solve::get_input;
        use crate::solve::print_answer;
        let input = get_input($year_number, $puzzle_number);
        let parsed = $parse(&input);
        let result_1 = $task_1(&parsed);
        print_answer($puzzle_number, &1, &result_1);
        let result_2 = $task_2(&parsed);
        print_answer($puzzle_number, &2, &result_2);
    }};
}
