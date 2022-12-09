use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 3;
    let year = 2022;
    solve!(&year, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    data.rucksacks
        .clone()
        .into_iter()
        .map(|sack| has_in_common(sack.0, sack.1))
        .flat_map(|chars| chars.into_iter().map(|c| to_priority(c).unwrap()))
        .sum::<usize>()
        .to_string()
}

fn has_in_common(a: &str, b: &str) -> Vec<char> {
    a.chars().filter(|c| b.contains(*c)).dedup().collect()
}

fn to_priority(s: char) -> Option<usize> {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();

    let index = alphabet.find(s);

    if let Some(i) = index {
        return Some(i + 1);
    } else {
        None
    }
}

fn task_2(data: &ParsedInput) -> String {
    data.rucksacks
        .clone()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .into_iter()
                .map(|sack| {
                    let mut res: String = String::from(sack.0); // There must be a better way
                    res.push_str(sack.1);
                    res
                })
                .reduce(|acc, sack| has_in_common(acc.as_str(), sack.as_str()).iter().join(""))
                .unwrap()
        })
        .map(|s| s.chars().map(|c| to_priority(c).unwrap()).sum::<usize>())
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]

struct ParsedInput<'a> {
    rucksacks: Vec<(&'a str, &'a str)>,
}

fn parse(input: &str) -> ParsedInput {
    let rucksacks = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect_vec();
    ParsedInput { rucksacks }
}

#[test]
fn test() {
    let test_input = String::from(
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "157");
    assert_eq!(task_2(&parsed), "70");
}
