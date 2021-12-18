use crate::solve;
use itertools::Itertools;
use std::cmp::Ordering;

pub fn run() {
    let day_number = 17;
    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let search_space = 300;
    let mut ys = vec![];
    let mut attempts = vec![];
    for i in 0..search_space {
        for j in 0..search_space {
            attempts.push((i, j))
        }
    }

    for attempt in attempts {
        if let Some(success) = launch(&data.area, attempt) {
            ys.push(success)
        }
    }

    ys.iter()
        .sorted()
        .rev()
        .collect_vec()
        .first()
        .unwrap()
        .to_string()
}

fn launch(area: &Area, mut velocity: (i32, i32)) -> Option<i32> {
    let mut position = (0, 0);
    let mut positions = vec![];
    while !area.has_overshot(position) {
        positions.push(position);
        if area.contains(position) {
            return Some(
                *positions
                    .iter()
                    .map(|p| p.1)
                    .sorted()
                    .rev()
                    .collect_vec()
                    .first()
                    .unwrap(),
            );
        }

        position.0 += velocity.0;
        position.1 += velocity.1;

        velocity.0 = match velocity.0.cmp(&0) {
            Ordering::Equal => 0,
            Ordering::Greater => velocity.0 - 1,
            Ordering::Less => velocity.0 + 1,
        };
        velocity.1 -= 1;
    }

    None
}

fn task_2(data: &ParsedInput) -> String {
    let search_space = 250;
    let mut ys = vec![];
    let mut attempts = vec![];

    for i in 0..search_space {
        for j in -search_space..search_space {
            attempts.push((i, j))
        }
    }

    for attempt in attempts {
        if let Some(success) = launch(&data.area, attempt) {
            ys.push(success)
        }
    }
    ys.len().to_string()
}

#[derive(Debug)]
struct Area {
    x: (i32, i32),
    y: (i32, i32),
}

impl Area {
    fn contains(&self, pos: (i32, i32)) -> bool {
        pos.0 >= self.x.0 && pos.0 <= self.x.1 && pos.1 >= self.y.0 && pos.1 <= self.y.1
    }

    fn has_overshot(&self, (x, y): (i32, i32)) -> bool {
        let b1 = self.x.1 < 0 && x < self.x.1;
        let b2 = self.x.1 > 0 && x > self.x.1;
        let b3 = self.y.0 > 0 && y > self.y.0;
        let b4 = self.y.0 < 0 && y < self.y.0;

        b4 || b3 || b2 || b1
    }
}

#[derive(Debug)]
struct ParsedInput {
    area: Area,
}

fn parse(input: &str) -> ParsedInput {
    let (x1, x2, y1, y2) = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
    ParsedInput {
        area: Area {
            x: (x1, x2),
            y: (y1, y2),
        },
    }
}

#[test]
fn test() {
    let test_input = String::from("20,30,-10,-5");
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "45");
    assert_eq!(task_2(&parsed), "112");
}
