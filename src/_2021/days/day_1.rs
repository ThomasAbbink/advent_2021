use itertools::Itertools;

use crate::solve;

pub fn run() {
    solve!(&2021, &1, parse, task_1, task_2)
}

fn count_increases(measurements: &[i32]) -> usize {
    measurements
        .iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn task_1(input: &ParsedInput) -> String {
    let count = count_increases(&input.depths);
    count.to_string()
}

fn task_2(input: &ParsedInput) -> String {
    let sums = input
        .depths
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect_vec();

    let count = count_increases(&sums);
    count.to_string()
}

struct ParsedInput {
    depths: Vec<i32>,
}

fn parse(data: &str) -> ParsedInput {
    let depths = data
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    ParsedInput { depths }
}
