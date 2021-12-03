use itertools::Itertools;
use std::fs;

pub fn run() {
    let data = get_data();
    task_1(&data);
    task_2(&data);
}

fn count_increases(measurements: &Vec<i32>) -> usize {
    measurements
        .iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn task_1(input: &ParsedInput) {
    let count = count_increases(&input.depths);
    println!("The count for task 1 is {}", count)
}

fn task_2(input: &ParsedInput) {
    let sums = input
        .depths
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect();

    let count = count_increases(&sums);
    println!("The count for task 2 is {}", count)
}

struct ParsedInput {
    depths: Vec<i32>,
}

fn get_data() -> ParsedInput {
    let path = "./src/days/day_1_input.txt";
    let data = fs::read_to_string(path).expect("unable to open file");
    let depths = data
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    ParsedInput { depths }
}
