use crate::solve;
use itertools::Itertools;

pub fn run() {
    let day_number = 10;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let mut illegals = vec![];
    for line in &data.lines {
        let mut owed = vec![];
        for ch in line {
            match *ch {
                "(" => owed.push(")"),
                "[" => owed.push("]"),
                "{" => owed.push("}"),
                "<" => owed.push(">"),
                ")" | "]" | "}" | ">" => {
                    if owed.iter().last().unwrap() != ch {
                        illegals.push(*ch);
                        break;
                    } else {
                        owed.pop();
                    }
                }
                _ => {}
            }
        }
    }

    calculate_syntax_checker_score(illegals).to_string()
}

fn calculate_syntax_checker_score(chars: Vec<&str>) -> i32 {
    chars
        .iter()
        .map(|c| match *c {
            ")" => 3,
            "]" => 57,
            "}" => 1197,
            ">" => 25137,
            _ => 0,
        })
        .sum::<i32>()
}

fn task_2(data: &ParsedInput) -> String {
    let mut unfinished = vec![];

    for line in &data.lines {
        let mut owed = vec![];
        for ch in line {
            match *ch {
                "(" => owed.push(")"),
                "[" => owed.push("]"),
                "{" => owed.push("}"),
                "<" => owed.push(">"),
                ")" | "]" | "}" | ">" => {
                    if owed.iter().last().unwrap() != ch {
                        owed.clear();
                        break;
                    } else {
                        owed.pop();
                    }
                }
                _ => {}
            }
        }
        unfinished.push(owed.clone());
    }
    let mut scores = calculate_autocomplete_score(
        unfinished
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| l.iter().rev().collect_vec())
            .collect_vec(),
    );
    scores.sort_unstable();
    scores[scores.len() / 2].to_string()
}

fn calculate_autocomplete_score(chars: Vec<Vec<&&str>>) -> Vec<i64> {
    chars
        .iter()
        .map(|line| {
            line.iter().fold(0, |acc, curr| {
                let char_score = match **curr {
                    ")" => 1,
                    "]" => 2,
                    "}" => 3,
                    ">" => 4,
                    _ => 0,
                };
                acc * 5 + char_score
            })
        })
        .collect_vec()
}

#[derive(Debug)]
struct ParsedInput<'a> {
    lines: Vec<Vec<&'a str>>,
}

fn parse(input: &str) -> ParsedInput {
    let lines = input
        .lines()
        .map(|line| line.split("").filter(|s| !s.is_empty()).collect_vec())
        .collect_vec();

    ParsedInput { lines }
}

#[test]
fn test() {
    let test_input = String::from(
        "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "26397");
    assert_eq!(task_2(&parsed), "288957");
}
