use crate::solve;
use std::cmp::Ordering;

pub fn run() {
    let day_number = 2;
    let year = 2022;
    solve!(&year, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    data.rounds
        .clone()
        .into_iter()
        .map(|round| score(round))
        .sum::<usize>()
        .to_string()
}

// 1 = rock
// 2 = paper
// 3 = scissors

fn score(round: (usize, usize)) -> usize {
    if round.1.cmp(&round.0) == Ordering::Equal {
        return round.1 + 3;
    }

    round.1
        + match round.1 {
            1 => {
                match round.0 {
                    3 => 6, // rock beats scissors
                    _ => 0,
                }
            }

            2 => {
                match round.0 {
                    1 => 6, // paper beats rock
                    _ => 0,
                }
            }
            3 => match round.0 {
                2 => 6, // scissors beats paper
                _ => 0,
            },
            _ => 0,
        }
}

// 1 = rock
// 2 = paper
// 3 = scissors

// 1 = lose (0)
// 2 = draw (3)
// 3 = win (6)
fn score_2(round: (usize, usize)) -> usize {
    let win_score = match round.1 {
        2 => 3,
        3 => 6,
        _ => 0,
    };

    let pick_score = match round.0 {
        1 => {
            // he picked rock
            match round.1 {
                1 => 3, // lose by picking scissors
                2 => 1, // tie by picking rock
                3 => 2, // win by picking paper
                _ => panic!(),
            }
        }
        2 => {
            // he picked paper
            match round.1 {
                1 => 1, // lose by picking rock
                2 => 2, // tie by picking paper
                3 => 3, // win by picking scissors
                _ => panic!(),
            }
        }
        3 => {
            // he picked scissors
            match round.1 {
                1 => 2, // lose by picking paper
                2 => 3, // tie by picking scissors
                3 => 1, // win by picking rock
                _ => panic!(),
            }
        }
        _ => panic!(),
    };

    win_score + pick_score
}

fn task_2(data: &ParsedInput) -> String {
    data.rounds
        .clone()
        .into_iter()
        .map(|round| score_2(round))
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
struct ParsedInput {
    rounds: Vec<(usize, usize)>,
}

fn parse(input: &str) -> ParsedInput {
    let rounds = input
        .lines()
        .map(|line| {
            let chars: Vec<usize> = line
                .trim()
                .split(" ")
                .map(|c| match c {
                    "X" | "A" => 1,
                    "Y" | "B" => 2,
                    "Z" | "C" => 3,
                    _ => panic!(),
                })
                .collect();
            (chars[0], chars[1])
        })
        .collect();
    ParsedInput { rounds }
}

#[test]
fn test() {
    let test_input = String::from(
        "A Y
B X
C Z",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "15");
    assert_eq!(task_2(&parsed), "12");
}
