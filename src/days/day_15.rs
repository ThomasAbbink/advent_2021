use crate::solve;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

pub fn run() {
    let day_number = 15;
    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let clone = data.nodes.to_owned();
    iterate(clone).to_string()
}

fn iterate(mut nodes: HashMap<(usize, usize), Node>) -> usize {
    // assuming the grid is a square.
    let grid_width = (nodes.len() as f64).sqrt() as usize - 1;
    let grid_size = (grid_width, grid_width);

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    heap.push(Node {
        xy: (0, 0),
        distance: 0,
        value: 0,
        visited: false,
    });

    while let Some(current_node) = heap.pop() {
        if current_node.xy == grid_size {
            return current_node.distance;
        }

        if current_node.visited {
            continue;
        }
        nodes.entry(current_node.xy).or_default().visited = true;
        // look at all its neighbors that are not yet visited
        let adjacent_xy = get_adjacent(current_node.xy, grid_size);

        for xy in adjacent_xy {
            let cost = nodes.get_mut(&xy).unwrap().value;
            let distance = nodes.get_mut(&xy).unwrap().distance;
            let distance_through_current = cost + current_node.distance;

            if distance_through_current < distance {
                // push it on the queue
                nodes.entry(xy).or_default().distance = distance_through_current;
                if !nodes.get(&xy).unwrap().visited {
                    heap.push(Node {
                        xy,
                        distance: distance_through_current,
                        visited: false,
                        value: cost,
                    })
                }
            }
        }
    }
    // we are done. return the value of the target node
    nodes.get(&grid_size).unwrap().distance
}

fn get_adjacent((x, y): (usize, usize), (w, h): (usize, usize)) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = vec![];

    // top
    if x > 0 {
        res.push((x - 1, y))
    }

    // bottom
    if x < h {
        res.push((x + 1, y))
    }

    // right
    if y < w {
        res.push((x, y + 1))
    };

    // left
    if y > 0 {
        res.push((x, y - 1))
    };

    res
}

fn task_2(data: &ParsedInput) -> String {
    let clone = data.times_5.to_owned();
    iterate(clone).to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Node {
    xy: (usize, usize),
    value: usize,
    distance: usize,
    visited: bool,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.xy.cmp(&other.xy))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct ParsedInput {
    nodes: HashMap<(usize, usize), Node>,
    times_5: HashMap<(usize, usize), Node>,
}

fn parse(input: &str) -> ParsedInput {
    let vec = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|l| !l.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let size = vec.len();
    let mut hm_times_5: HashMap<(usize, usize), Node> = HashMap::default();
    for i in 0..size * 5 {
        for j in 0..size * 5 {
            let orig = vec[i % size][j % size];
            // which square am i in?
            let row_num = i / size;
            let col_num = j / size;

            let val = orig + (row_num) + (col_num);
            let remainder = val % 10;
            let res = if remainder == val { val } else { remainder + 1 };

            hm_times_5.insert(
                (i, j),
                Node {
                    xy: (i, j),
                    value: res,
                    distance: usize::MAX,
                    visited: false,
                },
            );
        }
    }

    let w = vec.len();
    let h = vec.first().unwrap().len() as usize;
    let mut nodes: HashMap<(usize, usize), Node> = HashMap::default();

    for (x, _) in (0..w).enumerate() {
        for y in 0..h {
            nodes.insert(
                (x, y),
                Node {
                    xy: (x, y),
                    value: vec[x][y],
                    distance: usize::MAX,
                    visited: false,
                },
            );
        }
    }
    ParsedInput {
        nodes,
        times_5: hm_times_5,
    }
}

#[test]
fn test() {
    let test_input = String::from("1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581");

    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "40");
    assert_eq!(task_2(&parsed), "315");
}
