use std::cmp::Ordering;

use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 9;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let low_points = find_low_points(&data.height_map);
    (sum_locations(&low_points, &data.height_map) + low_points.len() as i32).to_string()
}

fn sum_locations(locations: &Vec<(usize, usize)>, height_map: &Vec<Vec<i32>>) -> i32 {
    locations
        .iter()
        .map(|(i, j)| height_map[*i][*j])
        .sum::<i32>()
}

fn find_low_points(height_map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut low_points: Vec<(usize, usize)> = vec![];

    for (i, map) in height_map.iter().enumerate() {
        for (j, val) in map.iter().enumerate() {
            let adjacent = get_adjacent(i, j, height_map);
            let values = adjacent
                .iter()
                .map(|point| get_value(point, height_map))
                .collect_vec();
            match values.iter().min() {
                Some(x) => {
                    if val < x {
                        low_points.push((i, j));
                    }
                }
                None => {}
            }
        }
    }
    low_points
}

fn get_adjacent(i: usize, j: usize, height_map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let w = height_map[0].len() - 1;
    let h = height_map.len() - 1;
    let mut res: Vec<(usize, usize)> = vec![];

    // top
    if i > 0 {
        res.push((i - 1, j))
    }

    // bottom
    if i < h {
        res.push((i + 1, j))
    }

    // right
    if j < w {
        res.push((i, j + 1))
    };

    // left
    if j > 0 {
        res.push((i, j - 1))
    };

    res
}

fn get_value((i, j): &(usize, usize), height_map: &Vec<Vec<i32>>) -> i32 {
    height_map[*i][*j]
}

fn expand_to_basin(basin: &Vec<(usize, usize)>, height_map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut new_locations = vec![];
    for (i, j) in basin {
        let adjacent = get_adjacent(*i, *j, height_map);
        for adjacent_point in adjacent {
            if !basin.contains(&adjacent_point) && get_value(&adjacent_point, height_map) != 9 {
                new_locations.push(adjacent_point);
            }
        }
    }

    if new_locations.is_empty() {
        basin.to_vec()
    } else {
        let mut b = basin.clone();
        b.append(&mut new_locations.into_iter().unique().collect_vec());
        expand_to_basin(&b, height_map)
    }
}

fn task_2(data: &ParsedInput) -> String {
    let low_points = find_low_points(&data.height_map);
    let mut basins = low_points
        .iter()
        .map(|loc| expand_to_basin(&vec![*loc], &data.height_map))
        .collect_vec();

    basins.sort_by(|a, b| {
        if a.len() < b.len() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let mut answer = 1;
    for i in 0..=2 {
        answer *= basins[i].len();
    }
    answer.to_string()
}

#[derive(Debug)]
struct ParsedInput {
    height_map: Vec<Vec<i32>>,
}
fn parse(input: &String) -> ParsedInput {
    let height_map = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    ParsedInput { height_map }
}

#[test]
fn test() {
    let test_input = String::from("2199943210\n3987894921\n9856789892\n8767896789\n9899965678");
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "15");
    assert_eq!(task_2(&parsed), "1134");
}
