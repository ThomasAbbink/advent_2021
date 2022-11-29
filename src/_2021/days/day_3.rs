use std::cmp::Ordering;

use crate::solve;
use itertools::Itertools;

pub fn run() {
    solve!(&2021, &3, parse, task_1, task_2);
}

fn task_1(numbers: &[String]) -> String {
    let byte_array = to_byte_array(numbers);
    let most_common = most_common(&byte_array, false);
    let least_common = invert(&most_common);
    let gamma = bytes_to_num(&most_common);
    let epsilon = bytes_to_num(&least_common);
    (gamma * epsilon).to_string()
}

fn bytes_to_num(bytes: &[i32]) -> isize {
    let str = bytes.iter().join("");
    isize::from_str_radix(&str, 2).unwrap()
}

fn to_byte_array(numbers: &[String]) -> Vec<Vec<i32>> {
    numbers
        .iter()
        .map(|str| {
            str.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect_vec()
}

fn most_common(numbers: &[Vec<i32>], is_inverted: bool) -> Vec<i32> {
    let matcher = numbers
        .iter()
        // count instances of 0 and 1s
        .fold(vec![0; 12], |acc: Vec<i32>, row| {
            row.iter()
                .enumerate()
                .map(|(index, value)| match value {
                    0 => acc[index] - 1,
                    1 => acc[index] + 1,
                    _ => acc[index],
                })
                .collect_vec()
        })
        .iter()
        .map(|num| match num.cmp(&0) {
            Ordering::Equal => 1,
            Ordering::Greater => 1,
            Ordering::Less => 0,
        })
        .collect_vec();

    if is_inverted {
        invert(&matcher)
    } else {
        matcher
    }
}

fn invert(vec: &[i32]) -> Vec<i32> {
    vec.iter()
        .map(|num| if num == &0 { 1 } else { 0 })
        .collect::<Vec<i32>>()
}

fn find_match(numbers: &[Vec<i32>], is_inverted: bool, iteration: usize) -> isize {
    let matcher = most_common(numbers, is_inverted);

    if numbers.len() == 1 {
        bytes_to_num(&numbers[0])
    } else {
        let next = numbers
            .iter()
            .filter(|&num| num[iteration] == matcher[iteration])
            .cloned()
            .collect::<Vec<Vec<i32>>>();
        find_match(&next, is_inverted, iteration + 1)
    }
}

fn task_2(numbers: &[String]) -> String {
    let byte_array = to_byte_array(numbers);

    let oxygen = find_match(&byte_array, false, 0);
    let co2 = find_match(&byte_array, true, 0);

    (oxygen * co2).to_string()
}

fn parse(data: &str) -> Vec<String> {
    data.lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect()
}

#[test]
fn test() {
    let data = String::from(
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
    );
    let numbers = parse(&data);
    assert_eq!(task_1(&numbers), "198");
    assert_eq!(task_2(&numbers), "230");
}
