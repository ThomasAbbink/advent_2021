use std::collections::HashMap;

use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 6;

    solve!(&day_number, parse, task_1, task_2);
}
#[derive(Debug, Clone)]
struct LanternFish {
    days_until_spawn: i32,
}

impl LanternFish {}

struct Ocean {
    weird_fishes: HashMap<i32, i64>,
}

impl Ocean {
    fn iterate(&mut self) {
        // shift down one day for all
        let mut next_iteration = self
            .weird_fishes
            .iter()
            .map(|(key, val)| ((key - 1), *val))
            .collect::<HashMap<i32, i64>>();

        let spawn_count = next_iteration.get(&-1).unwrap().clone();
        let sixes: i64 = match next_iteration.get(&6) {
            Some(x) => x.clone(),
            None => 0,
        };
        // take all -1 keys, put new fishes in 8, reset the cycle for all -1s
        next_iteration.remove(&-1);
        next_iteration.insert(8, spawn_count);
        next_iteration.insert(6, spawn_count + sixes);
        self.weird_fishes = next_iteration;
    }

    fn from(wf: &Vec<LanternFish>) -> Ocean {
        let mut hm = HashMap::new();
        for i in 0..8 {
            hm.insert(i, 0);
        }

        for fish in wf {
            hm.insert(
                fish.days_until_spawn,
                hm.get(&fish.days_until_spawn).unwrap() + 1,
            );
        }
        Ocean { weird_fishes: hm }
    }

    fn get_fish_count(&self) -> i64 {
        self.weird_fishes.values().sum()
    }
}

fn task_1(data: &ParsedInput) -> String {
    let mut ocean = Ocean::from(&data.weird_fishes);

    for _i in 0..80 {
        ocean.iterate()
    }
    ocean.get_fish_count().to_string()
}

fn task_2(data: &ParsedInput) -> String {
    let mut ocean = Ocean::from(&data.weird_fishes);

    for _i in 0..256 {
        ocean.iterate()
    }
    ocean.get_fish_count().to_string()
}

struct ParsedInput {
    weird_fishes: Vec<LanternFish>,
}

fn parse(input: &String) -> ParsedInput {
    ParsedInput {
        weird_fishes: input
            .split(",")
            .map(|s| LanternFish {
                days_until_spawn: s.parse::<i32>().unwrap(),
            })
            .collect_vec(),
    }
}

#[test]
fn test() {
    let test_input = String::from("3,4,3,1,2");
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "5934");
    assert_eq!(task_2(&parsed), "26984457539");
}
