use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 1;
    let year = 2022;
    solve!(&year, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    data.calories
        .clone()
        .into_iter()
        .map(|val| val.into_iter().sum::<usize>())
        .max()
        .unwrap()
        .to_string()
}

fn task_2(data: &ParsedInput) -> String {
    data.calories
        .clone()
        .into_iter()
        .map(|val| val.into_iter().sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
struct ParsedInput {
    calories: Vec<Vec<usize>>,
}

fn parse(input: &str) -> ParsedInput {
    let mut calories: Vec<Vec<usize>> = vec![vec![]];
    for line in input.lines() {
        match line.parse::<usize>() {
            Ok(num) => match calories.last_mut() {
                Some(last) => last.push(num),
                None => {
                    let mut arr = vec![];
                    arr.push(num);
                    calories.push(arr)
                }
            },
            Err(_) => calories.push(vec![]),
        };
    }
    ParsedInput { calories }
}

#[test]
fn test() {
    let test_input = String::from(
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "24000");
    assert_eq!(task_2(&parsed), "45000");
}
