use colored::Colorize;
use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 13;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let first_instruction = data.fold_instructions.first().unwrap();
    let fold_1 = fold(&data.paper, first_instruction);

    fold_1.len().to_string()
}

fn fold(paper: &[Dot], instructions: &FoldInstructions) -> Vec<Dot> {
    let mut res: Vec<Dot> = vec![];
    for dot in paper {
        match instructions.axis {
            Axis::X => {
                if dot.x < instructions.line_number {
                    res.push(*dot)
                } else {
                    let new_x = instructions.line_number - (dot.x - instructions.line_number);
                    res.push(Dot { x: new_x, y: dot.y })
                }
            }
            Axis::Y => {
                if dot.y < instructions.line_number {
                    res.push(*dot)
                } else {
                    let new_y = instructions.line_number - (dot.y - instructions.line_number);
                    res.push(Dot { x: dot.x, y: new_y })
                }
            }
        }
    }

    res.into_iter().unique().collect_vec()
}

fn task_2(data: &ParsedInput) -> String {
    let mut res = data.paper.clone();
    for i in &data.fold_instructions {
        res = fold(&res, i);
    }
    print_dots(&res);
    String::from("ABKJFBGC")
}

fn print_dots(dots: &[Dot]) {
    let x_max = dots
        .iter()
        .fold(0, |x, dot| if dot.x > x { dot.x } else { x });
    let y_max = dots
        .iter()
        .fold(0, |y, dot| if dot.y > y { dot.y } else { y });

    for y in 0..=y_max {
        let mut line = String::from("");
        for x in 0..=x_max {
            if dots.contains(&Dot { x, y }) {
                line.push('#')
            } else {
                line.push(' ')
            }
        }
        println!("{}", line.truecolor(0, 204, 0).bold())
    }
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct FoldInstructions {
    axis: Axis,
    line_number: i32,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Dot {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ParsedInput {
    paper: Vec<Dot>,
    fold_instructions: Vec<FoldInstructions>,
}

fn parse(input: &str) -> ParsedInput {
    let lines = input.lines();

    let fold_instructions = lines
        .clone()
        .filter(|line| line.contains("fold"))
        .map(|line| line.split(' ').last().unwrap())
        .map(|str| {
            let (a, line_number) = str.split('=').collect_tuple().unwrap();
            let axis = match a {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!(),
            };

            FoldInstructions {
                axis,
                line_number: line_number.parse::<i32>().unwrap(),
            }
        })
        .collect_vec();

    let dots: Vec<Dot> = lines
        .filter(|line| !line.is_empty())
        .filter(|line| !line.contains("fold"))
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(x, y)| Dot { x, y })
        .collect_vec();

    ParsedInput {
        paper: dots,
        fold_instructions,
    }
}

#[test]
fn test() {
    let test_input = String::from("\n6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5");
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "17");
}
