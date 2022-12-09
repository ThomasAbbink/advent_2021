use crate::solve;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_TEST: AtomicBool = AtomicBool::new(false);

pub fn run() {
    let day_number = 5;
    let year = 2022;
    solve!(&year, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let mut c = data.crates.clone();
    for instruction in &data.instructions {
        for _i in 0..instruction.amount {
            move_crate(&mut c, instruction.from, instruction.to);
        }
    }
    get_answer(c)
}

fn get_answer(crates: Vec<Vec<&str>>) -> String {
    crates
        .into_iter()
        .map(|stack| stack.into_iter().last().unwrap())
        .collect()
}

fn move_crate(crates: &mut Vec<Vec<&str>>, from: usize, to: usize) {
    let to_move = crates[from - 1].pop().unwrap();
    crates[to - 1].push(to_move);
}

fn move_crates(crates: &mut Vec<Vec<&str>>, instruction: &Instruction) {
    let at = crates[instruction.from - 1].len() - instruction.amount;
    let to_move = crates[instruction.from - 1].split_off(at);
    crates[instruction.to - 1].extend_from_slice(&to_move);
}

fn task_2(data: &ParsedInput) -> String {
    let mut c = data.crates.clone();
    for instruction in &data.instructions {
        move_crates(&mut c, instruction);
    }
    get_answer(c)
}

fn get_crates<'a>() -> Vec<Vec<&'a str>> {
    if IS_TEST.load(Ordering::Relaxed) {
        return vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];
    }
    return vec![
        vec!["F", "D", "B", "Z", "T", "J", "R", "N"],
        vec!["R", "S", "N", "J", "H"],
        vec!["C", "R", "N", "J", "G", "Z", "F", "Q"],
        vec!["F", "V", "N", "G", "R", "T", "Q"],
        vec!["L", "T", "Q", "F"],
        vec!["Q", "C", "W", "Z", "B", "R", "G", "N"],
        vec!["F", "C", "L", "S", "N", "H", "M"],
        vec!["D", "N", "Q", "M", "T", "J"],
        vec!["P", "G", "S"],
    ];
}
#[derive(Debug)]
struct ParsedInput<'a> {
    instructions: Vec<Instruction>,
    crates: Vec<Vec<&'a str>>,
}

fn parse(input: &str) -> ParsedInput {
    let crates = get_crates();
    let instructions = input
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split(" ").collect();
            Instruction {
                amount: words[1].parse().unwrap(),
                from: words[3].parse().unwrap(),
                to: words[5].parse().unwrap(),
            }
        })
        .collect();
    ParsedInput {
        crates,
        instructions,
    }
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

#[test]
fn test() {
    let test_input = String::from(
        "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    );
    IS_TEST.store(true, Ordering::Relaxed);
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "CMZ");
    assert_eq!(task_2(&parsed), "MCD");
}
