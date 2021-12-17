use std::collections::HashMap;

use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 14;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    do_iterations(data, 10)
}

fn task_2(data: &ParsedInput) -> String {
    do_iterations(data, 40)
}

#[derive(Debug)]
struct Params {
    char_count: HashMap<char, u128>,
    pair_count: HashMap<String, u128>,
}

fn iterate(params: Params, rules: &HashMap<String, String>) -> Params {
    let mut pair_count: HashMap<String, u128> = params.pair_count.clone();

    let mut char_count = params.char_count.clone();
    for (original_pair, count) in params.pair_count {
        let in_between = rules.get(&original_pair).unwrap();
        let pair_1 = format!("{}{}", original_pair.chars().next().unwrap(), in_between);
        let pair_2 = format!("{}{}", in_between, original_pair.chars().nth(1).unwrap());

        *pair_count.entry(pair_1.clone()).or_insert(0) += count;
        *pair_count.entry(pair_2.clone()).or_insert(0) += count;

        *char_count
            .entry(in_between.chars().next().unwrap())
            .or_insert(0) += count;

        *pair_count.entry(original_pair.to_string()).or_insert(0) -= count;
    }
    Params {
        pair_count,
        char_count,
    }
}

fn do_iterations(data: &ParsedInput, iterations: i32) -> String {
    let mut pair_count: HashMap<String, u128> = HashMap::default();
    for (a, b) in data.template.chars().tuple_windows() {
        *pair_count.entry(format!("{}{}", a, b)).or_insert(0) += 1;
    }

    let mut char_count: HashMap<char, u128> = HashMap::default();
    for c in data.template.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    let mut res = Params {
        pair_count,
        char_count,
    };

    for _i in 0..iterations {
        res = iterate(res, &data.insertion_rules);
    }

    calculate_answer(&res.char_count).to_string()
}

fn calculate_answer(s: &HashMap<char, u128>) -> u128 {
    let counted = s.iter().sorted_by(|a, b| b.1.cmp(a.1)).collect_vec();
    let (_char, count) = counted.first().unwrap();
    let (_char_s, count_s) = counted.last().unwrap();
    *count - *count_s
}

#[derive(Debug)]
struct ParsedInput {
    template: String,
    insertion_rules: HashMap<String, String>,
}
fn parse(input: &str) -> ParsedInput {
    let lines = input.lines();
    let template = lines.clone().collect_vec().first().unwrap().to_string();

    let rules = lines
        .filter(|line| line.contains("->"))
        .map(|line| {
            line.split("->")
                .map(|s| String::from(s.trim()))
                .collect_tuple()
                .unwrap()
        })
        .collect::<HashMap<String, String>>();

    ParsedInput {
        template,
        insertion_rules: rules,
    }
}

#[test]
fn test() {
    let test_input = String::from(
        "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "1588");
    assert_eq!(task_2(&parsed), "2188189693529");
}
