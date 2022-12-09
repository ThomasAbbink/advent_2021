use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 4;
    let year = 2022;
    solve!(&year, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    data.sections
        .clone()
        .into_iter()
        .filter(|(a, b)| fully_contained(a, b) || fully_contained(b, a))
        .collect_vec()
        .len()
        .to_string()
}

fn fully_contained(a: &(usize, usize), b: &(usize, usize)) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn overlap(a: &(usize, usize), b: &(usize, usize)) -> bool {
    (a.0 >= b.0 && a.0 <= b.1) || (a.1 <= b.1 && a.1 >= b.0)
}

fn task_2(data: &ParsedInput) -> String {
    data.sections
        .clone()
        .into_iter()
        .filter(|(a, b)| overlap(a, b) || overlap(b, a))
        .collect_vec()
        .len()
        .to_string()
}

#[derive(Debug)]
struct ParsedInput {
    sections: Vec<((usize, usize), (usize, usize))>,
}

fn parse(input: &str) -> ParsedInput {
    let sections = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|pair| {
                    pair.split("-")
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect_tuple::<(usize, usize)>()
                        .unwrap()
                })
                .collect_tuple::<((usize, usize), (usize, usize))>()
                .unwrap()
        })
        .collect_vec();
    ParsedInput { sections }
}

#[test]
fn test() {
    let test_input = String::from(
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "2");
    assert_eq!(task_2(&parsed), "4");
}
