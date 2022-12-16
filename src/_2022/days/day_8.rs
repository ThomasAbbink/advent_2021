use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 8;
    let year = 2022;
    solve!(&year, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let mut count = 0;

    let transposed = transpose(&data.clone().lines);
    for i in 0..data.lines.len() {
        for j in 0..data.lines[0].len() {
            if is_visible(&data.lines, &transposed, (i, j)) {
                count += 1;
            }
        }
    }
    count.to_string()
}

fn task_2(data: &ParsedInput) -> String {
    let transposed = transpose(&data.clone().lines);
    let mut counts = vec![];
    for i in 0..data.lines.len() {
        for j in 0..data.lines[0].len() {
            counts.push(scenic_score(&data.lines, &transposed, (i, j)))
        }
    }

    counts.iter().max().unwrap().to_string()
}

#[derive(Debug)]
struct ParsedInput {
    lines: Vec<Vec<usize>>,
}

fn is_visible(grid: &Vec<Vec<usize>>, transposed: &Vec<Vec<usize>>, pos: (usize, usize)) -> bool {
    if pos.0 == 0 || pos.1 == 0 || pos.0 == grid[0].len() - 1 || pos.1 == grid.len() - 1 {
        return true;
    }
    let val = grid[pos.0][pos.1];

    let max_size = &grid[0].len();

    let from_left = &grid[pos.0][0..(pos.1)];
    let from_right = &grid[pos.0][(pos.1 + 1)..*max_size];
    let from_top = &transposed[pos.1][0..(pos.0)];
    let from_bottom = &transposed[pos.1][(pos.0 + 1)..*max_size];

    let is_visible_from_left = val > *from_left.iter().max().unwrap_or(&0);
    let is_visible_from_right = val > *from_right.iter().max().unwrap_or(&0);
    let is_visible_from_top = val > *from_top.iter().max().unwrap_or(&0);
    let is_visible_from_bottom = val > *from_bottom.iter().max().unwrap_or(&0);

    is_visible_from_left || is_visible_from_right || is_visible_from_top || is_visible_from_bottom
}

fn scenic_score(
    grid: &Vec<Vec<usize>>,
    transposed: &Vec<Vec<usize>>,
    pos: (usize, usize),
) -> usize {
    if pos.0 == 0 || pos.1 == 0 || pos.0 == grid[0].len() - 1 || pos.1 == grid.len() - 1 {
        return 0;
    }
    let val = grid[pos.0][pos.1];

    let max_size = &grid[0].len();

    let from_left = &grid[pos.0][0..(pos.1)].iter().rev().collect_vec();

    let from_right = &grid[pos.0][(pos.1 + 1)..*max_size].iter().collect_vec();

    let from_top = &transposed[pos.1][0..(pos.0)].iter().rev().collect_vec();

    let from_bottom = &transposed[pos.1][(pos.0 + 1)..*max_size]
        .iter()
        .collect_vec();

    count_visible_from(from_left, &val)
        * count_visible_from(from_right, &val)
        * count_visible_from(from_top, &val)
        * count_visible_from(from_bottom, &val)
}

fn count_visible_from(vec: &Vec<&usize>, tree_size: &usize) -> usize {
    let count = vec
        .iter()
        .take_while(|i| *i < &tree_size)
        .collect_vec()
        .len();

    if count == vec.len() {
        count
    } else {
        count + 1
    }
}

fn transpose<T>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..vec[0].len())
        .map(|i| vec.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn parse(input: &str) -> ParsedInput {
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();
    ParsedInput { lines }
}

#[test]
fn test() {
    let test_input = String::from(
        "30373
25512
65332
33549
35390",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "21");
    assert_eq!(task_2(&parsed), "8");
}
