pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;

fn get_input(day_number: &i32) -> String {
    use std::fs;
    let path = format!("./src/input/day_{}.txt", day_number);
    fs::read_to_string(path).expect("unable to open file")
}

fn print_answer(day_number: &i32, task_number: &i32, result: &String) {
    println!(
        "The answer to day {} task {} is {}",
        day_number, task_number, result
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
