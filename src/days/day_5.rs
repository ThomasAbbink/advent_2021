use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 5;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let locations = data
        .lines
        .iter()
        .filter(|line| line.ends[0].x == line.ends[1].x || line.ends[0].y == line.ends[1].y)
        .map(|line| line.get_locations())
        .flatten()
        .collect_vec();
    let mut duplicates = vec![];
    for point in &locations {
        let count = locations
            .iter()
            .filter(|p| p.x == point.x && p.y == point.y)
            .count();
        if count >= 2 {
            duplicates.push(point)
        }
    }
    (duplicates.iter().unique().collect_vec().len()).to_string()
}

fn task_2(data: &ParsedInput) -> String {
    let locations = data
        .lines
        .iter()
        .map(|line| line.get_locations())
        .flatten()
        .collect_vec();
    let mut duplicates = vec![];
    for point in &locations {
        let count = locations
            .iter()
            .filter(|p| p.x == point.x && p.y == point.y)
            .count();
        if count >= 2 {
            duplicates.push(point)
        }
    }
    (duplicates.iter().unique().collect_vec().len()).to_string()
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
    let next_x = if target.x > location.x {
        location.x + 1
    } else if target.x < location.x {
        location.x - 1
    } else {
        location.x
    };

    let next_y = if target.y > location.y {
        location.y + 1
    } else if target.y < location.y {
        location.y - 1
    } else {
        location.y
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
            let x_y = point.trim().split(",").collect_vec();
            Point {
                x: x_y[0].parse::<i32>().unwrap(),
                y: x_y[1].parse::<i32>().unwrap(),
            }
        })
        .collect_vec();

    Line { ends: s }
}

fn parse(input: &String) -> ParsedInput {
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
