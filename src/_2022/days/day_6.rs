use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 6;
    let year = 2022;
    solve!(&year, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let mut s = vec![];
    for (i, c) in data.signal.chars().enumerate() {
        if s.len() > 3 {
            let mut last_4 = s[s.len() - 4..].into_iter().collect_vec();
            last_4.sort();
            last_4.dedup();
            if last_4.len() == 4 {
                return (i).to_string();
            }
        }
        s.push(c);
    }

    String::from("No signal found!")
}

fn task_2(data: &ParsedInput) -> String {
    let mut s = vec![];
    for (i, c) in data.signal.chars().enumerate() {
        if s.len() > 13 {
            let mut last_4 = s[s.len() - 14..].into_iter().collect_vec();
            last_4.sort();
            last_4.dedup();
            if last_4.len() == 14 {
                return (i).to_string();
            }
        }
        s.push(c);
    }

    String::from("No signal found!")
}
#[derive(Debug)]
struct ParsedInput<'a> {
    signal: &'a str,
}

fn parse(input: &str) -> ParsedInput {
    ParsedInput { signal: input }
}

#[test]
fn test() {
    let test_input = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "7");
    assert_eq!(task_2(&parsed), "19");
}
