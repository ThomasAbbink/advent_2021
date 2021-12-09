use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 5;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let locations = create_locations(data, |line: &Line| {
        line.ends[0].x == line.ends[1].x || line.ends[0].y == line.ends[1].y
    });
    count_locations(&locations).to_string()
}

fn task_2(data: &ParsedInput) -> String {
    let locations = create_locations(data, |_line: &Line| true);
    count_locations(&locations).to_string()
}

fn create_locations(data: &ParsedInput, filter: fn(&Line) -> bool) -> Vec<Point> {
    data.lines
        .iter()
        .filter(|line| filter(line))
        .map(|line| line.get_locations())
        .flatten()
        .collect_vec()
}

fn count_locations(locations: &[Point]) -> i32 {
    let mut counts: HashMap<Point, i32> = Default::default();
    for point in locations {
        *counts.entry(*point).or_insert(0) += 1;
    }
    counts.iter().filter(|(_, count)| **count > 1).count() as i32
}

#[derive(Debug)]

struct ParsedInput {
    lines: Vec<Line>,
}

#[derive(Debug)]
struct Line {
    ends: Vec<Point>,
}

impl Line {
    fn get_locations(&self) -> Vec<Point> {
        let mut points = vec![self.ends[0]];
        let mut curr = self.ends[0];

        while curr.x != self.ends[1].x || curr.y != self.ends[1].y {
            let next = move_to_target(&curr, &self.ends[1]);
            points.push(next);
            curr = next;
        }

        points
    }
}

fn move_to_target(location: &Point, target: &Point) -> Point {
    let next_x = match target.x.cmp(&location.x) {
        Ordering::Greater => location.x + 1,
        Ordering::Less => location.x - 1,
        Ordering::Equal => location.x,
    };
    let next_y = match target.y.cmp(&location.y) {
        Ordering::Greater => location.y + 1,
        Ordering::Less => location.y - 1,
        Ordering::Equal => location.y,
    };

    Point {
        x: next_x,
        y: next_y,
    }
}
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_line(l: &str) -> Line {
    let s = l
        .split("->")
        .collect_vec()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|point| {
            let x_y = point.trim().split(',').collect_vec();
            Point {
                x: x_y[0].parse::<i32>().unwrap(),
                y: x_y[1].parse::<i32>().unwrap(),
            }
        })
        .collect_vec();

    Line { ends: s }
}

fn parse(input: &str) -> ParsedInput {
    let lines = input.lines().map(parse_line).collect_vec();

    ParsedInput { lines }
}

#[test]
fn test() {
    let test_input = String::from(
        "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "5");
    assert_eq!(task_2(&parsed), "12");
}
