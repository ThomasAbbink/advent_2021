use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 8;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let count = data
        .lines
        .iter()
        .map(|line| line.output_values.clone())
        .flatten()
        .filter(|val| match val.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count();

    count.to_string()
}

fn task_2(data: &ParsedInput) -> String {
    let res = data
        .lines
        .iter()
        .map(|line| SSDisplay {
            input_values: line.input_values.clone(),
            output_values: line.output_values.clone(),
        })
        .map(|mut display| display.digits());

    res.sum::<i32>().to_string()
}

struct SSDisplay {
    input_values: Vec<String>,
    output_values: Vec<String>,
}

impl SSDisplay {
    fn digits(&mut self) -> i32 {
        let model = self.model();
        let mut digits: Vec<i32> = vec![];
        for s in &self.output_values {
            let translated = self.translate(s, &model);
            let digit = panels_to_digit(&translated);
            digits.push(digit);
        }
        digits.iter().join("").parse::<i32>().unwrap()
    }

    fn translate(&self, string: &String, model: &HashMap<String, String>) -> String {
        string
            .split("")
            .map(String::from)
            .filter(|s| !s.is_empty())
            .map(|s| {
                let (k, _v) = model
                    .iter()
                    .find(|(_key, val)| val.to_string() == s)
                    .unwrap();
                k
            })
            .join("")
    }

    // pff this is terrible.
    fn model(&mut self) -> HashMap<String, String> {
        let mut options = vec!["a", "b", "c", "d", "e", "f", "g"]
            .iter()
            .map(|s| {
                (
                    String::from(s.clone()),
                    vec!["a", "b", "c", "d", "e", "f", "g"]
                        .iter()
                        .map(|s| String::from(*s))
                        .collect_vec(),
                )
            })
            .collect::<HashMap<String, Vec<String>>>();
        let char_count: HashMap<String, i32> = self
            .input_values
            .clone()
            .join("")
            .split("")
            .filter(|s| !s.is_empty())
            .fold(HashMap::new(), |mut acc, curr| {
                acc.insert(
                    curr.to_string(),
                    1 + if acc.contains_key(curr) { acc[curr] } else { 0 },
                );
                acc
            });
        // a: 8
        // b: 6
        // c: 8
        // d: 7
        // e: 4
        // f: 9
        // g: 7

        // some letters occur a unique total amount of times
        for (letter, count) in char_count {
            match count {
                6 => {
                    options.insert("b".to_string(), vec![letter]);
                }
                4 => {
                    options.insert("e".to_string(), vec![letter]);
                }
                9 => {
                    options.insert("f".to_string(), vec![letter]);
                }
                _ => {}
            }
        }

        self.input_values.sort_by(|a, b| {
            if a.chars().count() > b.chars().count() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        for s in &self.input_values {
            let letters = s
                .split("")
                .map(String::from)
                .filter(|s| !s.is_empty())
                .collect_vec();
            match s.chars().count() {
                2 => {
                    //1
                    let f_options = options.get("f").unwrap();
                    let c_mapping = letters
                        .iter()
                        .filter(|l| !f_options.contains(l))
                        .map(String::from)
                        .collect_vec();
                    // we already know f, so the other must be c
                    options.insert("c".to_string(), c_mapping);
                }
                3 => {
                    // 7
                    // we can find out the mapping for "a"
                    let c_options = options.get("c").unwrap();
                    let f_options = options.get("f").unwrap();

                    let a_mapping = letters
                        .iter()
                        .filter(|l| !c_options.contains(l))
                        .filter(|l| !f_options.contains(l))
                        .map(String::from)
                        .collect_vec();
                    options.insert("a".to_string(), a_mapping);
                }
                // by now we know a, b, c, e and f
                4 => {
                    // 4: we can figure out d
                    let c_options = options.get("c").unwrap();
                    let f_options = options.get("f").unwrap();
                    let b_options = options.get("b").unwrap();

                    let d_mapping = letters
                        .iter()
                        .filter(|l| !c_options.contains(l))
                        .filter(|l| !f_options.contains(l))
                        .filter(|l| !b_options.contains(l))
                        .map(String::from)
                        .collect_vec();
                    options.insert("d".to_string(), d_mapping);
                }
                _ => {}
            };
        }
        // only one left is g
        let a_options = options.get("a").unwrap();
        let b_options = options.get("b").unwrap();
        let c_options = options.get("c").unwrap();
        let d_options = options.get("d").unwrap();
        let e_options = options.get("e").unwrap();
        let f_options = options.get("f").unwrap();

        let g_mapping = options
            .get("g")
            .unwrap()
            .iter()
            .filter(|l| !a_options.contains(l))
            .filter(|l| !b_options.contains(l))
            .filter(|l| !c_options.contains(l))
            .filter(|l| !d_options.contains(l))
            .filter(|l| !e_options.contains(l))
            .filter(|l| !f_options.contains(l))
            .map(String::from)
            .collect_vec();

        options.insert("g".to_string(), g_mapping);
        let done = options
            .iter()
            .map(|(key, vec)| (key.to_string(), vec[0].to_string()))
            .collect::<HashMap<String, String>>();
        done
    }
}

fn panels_to_digit(string: &String) -> i32 {
    let mut panels = string
        .trim()
        .split("")
        .filter(|s| !s.is_empty())
        .collect_vec();
    panels.sort_unstable();

    match panels.as_slice() {
        ["a", "b", "c", "e", "f", "g"] => 0,
        ["c", "f"] => 1,
        ["a", "c", "d", "e", "g"] => 2,
        ["a", "c", "d", "f", "g"] => 3,
        ["b", "c", "d", "f"] => 4,
        ["a", "b", "d", "f", "g"] => 5,
        ["a", "b", "d", "e", "f", "g"] => 6,
        ["a", "c", "f"] => 7,
        ["a", "b", "c", "d", "e", "f", "g"] => 8,
        ["a", "b", "c", "d", "f", "g"] => 9,
        _ => {
            println!("could not parse {:?}", string);
            99
        }
    }
}

#[derive(Debug)]
struct Line {
    input_values: Vec<String>,
    output_values: Vec<String>,
}

struct ParsedInput {
    lines: Vec<Line>,
}

fn parse(input: &String) -> ParsedInput {
    let lines = input
        .lines()
        .map(|line| line.split('|').map(String::from).collect_vec())
        .filter(|line| line.len() > 1)
        .map(|line| Line {
            input_values: line[0]
                .trim()
                .split(' ')
                .map(String::from)
                .collect_vec(),
            output_values: line[1]
                .trim()
                .split(' ')
                .map(String::from)
                .collect_vec(),
        })
        .collect_vec();

    ParsedInput { lines }
}

#[test]
fn test() {
    let test_input = String::from(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    );

    let input_2 = String::from(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
    );
    let parsed_2 = parse(&input_2);
    assert_eq!(task_2(&parsed_2), "5353");

    let parsed = parse(&test_input);
    assert_eq!(panels_to_digit(&String::from("cf")), 1);
    assert_eq!(panels_to_digit(&String::from("fc")), 1);
    assert_eq!(panels_to_digit(&String::from("abcefg")), 0);
    assert_eq!(panels_to_digit(&String::from("gfdca")), 3);

    assert_eq!(task_1(&parsed), "26");
    assert_eq!(task_2(&parsed), "61229");
}
