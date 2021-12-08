use average::Mean;
use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 7;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(positions: &Vec<f64>) -> String {
    let min = positions.iter().map(|i| *i as i64).min().unwrap();
    let max = positions.iter().map(|i| *i as i64).max().unwrap();
    let mut calcs = vec![];
    for i in min..max {
        calcs.push(calculate_costs(positions, i as i32))
    }

    calcs.iter().min().unwrap().to_string()
}

fn calculate_costs(positions: &Vec<f64>, target: i32) -> i64 {
    positions
        .iter()
        .map(|f| f.round() as i64)
        .map(|i| (i - target as i64).abs())
        .sum()
}

fn calculate_costs_2(positions: &Vec<f64>, target: i32) -> i64 {
    positions
        .iter()
        .map(|f| f.round() as i64)
        .map(|i| {
            let distance = (i - target as i64).abs();
            (1..=distance).sum::<i64>()
        })
        .sum()
}

fn task_2(positions: &Vec<f64>) -> String {
    let mean: Mean = positions.iter().collect();
    let min = mean.mean() as i32 - 1;
    let max = mean.mean() as i32 + 1;
    let mut calcs = vec![];
    for i in min..max {
        calcs.push(calculate_costs_2(positions, i as i32))
    }

    calcs.iter().min().unwrap().to_string()
}

fn parse(input: &String) -> Vec<f64> {
    input
        .split(",")
        .map(|s| s.parse::<f64>().unwrap())
        .collect_vec()
}

#[test]
fn test() {
    let test_input = String::from("16,1,2,0,4,2,7,1,2,14");
    let parsed = parse(&test_input);
    assert_eq!(calculate_costs(&parsed, 1), 41);
    assert_eq!(calculate_costs(&parsed, 3), 39);
    assert_eq!(calculate_costs(&parsed, 10), 71);

    assert_eq!(calculate_costs_2(&parsed, 5), 168);
    assert_eq!(calculate_costs_2(&parsed, 2), 206);
    assert_eq!(task_1(&parsed), "37");

    assert_eq!(task_2(&parsed), "168");
}
