use itertools::Itertools;
use ndarray::{s, Array2};

use crate::solve;

pub fn run() {
    let day_number = 11;

    solve!(&2021, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let mut arr: Array2<i32> =
        Array2::from_shape_vec((10, 10), data.rows.iter().flatten().collect_vec())
            .unwrap()
            .map(|s| **s);

    let mut count = 0;
    for _i in 0..100 {
        arr.mapv_inplace(|f| if f < 10 { f + 1 } else { f });

        loop {
            let next_nine = arr.iter().enumerate().find(|(_, s)| **s == 10);

            match next_nine {
                Some((index, mut _element)) => {
                    count += 1;
                    let row_num = index as i32 / 10;
                    let col_num = index as i32 % 10;
                    // find the neighbors
                    // add 1 if it is not already a 9
                    arr.slice_mut(s![
                        (row_num - 1).clamp(0, 9)..=(row_num + 1).clamp(0, 9),
                        (col_num - 1).clamp(0, 9)..=(col_num + 1).clamp(0, 9)
                    ])
                    .mapv_inplace(|f| if f < 10 { f + 1 } else { f });

                    // make the current 10  an 11 to indicate it has flashed
                    arr[[row_num as usize, col_num as usize]] = 11;
                }
                None => {
                    arr.mapv_inplace(|v| if v == 11 { 0 } else { v });
                    break;
                }
            }
        }
    }

    count.to_string()
}

fn task_2(data: &ParsedInput) -> String {
    let mut arr: Array2<i32> =
        Array2::from_shape_vec((10, 10), data.rows.iter().flatten().collect_vec())
            .unwrap()
            .map(|s| **s);

    let mut count = 0;
    loop {
        let all_synced = arr.iter().all(|f| *f == 0);
        if all_synced {
            break;
        }
        count += 1;
        arr.mapv_inplace(|f| if f < 10 { f + 1 } else { f });

        loop {
            let next_nine = arr.iter().enumerate().find(|(_, s)| **s == 10);

            match next_nine {
                Some((index, mut _element)) => {
                    let row_num = index as i32 / 10;
                    let col_num = index as i32 % 10;
                    // find the neighbors
                    // add 1 if it is not already a 9
                    arr.slice_mut(s![
                        (row_num - 1).clamp(0, 9)..=(row_num + 1).clamp(0, 9),
                        (col_num - 1).clamp(0, 9)..=(col_num + 1).clamp(0, 9)
                    ])
                    .mapv_inplace(|f| if f < 10 { f + 1 } else { f });

                    // make the current 10  an 11 to indicate it has flashed
                    arr[[row_num as usize, col_num as usize]] = 11;
                }
                None => {
                    arr.mapv_inplace(|v| if v == 11 { 0 } else { v });
                    break;
                }
            }
        }
    }
    count.to_string()
}

#[derive(Debug)]
struct ParsedInput {
    rows: Vec<Vec<i32>>,
}

fn parse(input: &str) -> ParsedInput {
    let lines = input
        .lines()
        .map(|l| {
            l.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    ParsedInput { rows: lines }
}

#[test]
fn test() {
    let test_input = String::from(
        "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "1656");
    assert_eq!(task_2(&parsed), "195");
}
