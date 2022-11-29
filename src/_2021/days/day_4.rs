use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 4;

    solve!(&2021, &day_number, parse, task_1, task_2);
}
#[derive(Debug)]
struct BingoInput {
    boards: Vec<BingoBoard>,
    drawn_numbers: Vec<i32>,
}

#[derive(Debug)]
struct BingoBoard {
    numbers: Vec<i32>,
    size: usize,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

impl BingoBoard {
    fn is_winning(&self, drawn_numbers: &[&i32]) -> bool {
        self.rows()
            .iter()
            .chain(self.columns().iter())
            .any(|row| row.iter().all(|num| drawn_numbers.contains(&num)))
    }

    fn rows(&self) -> Vec<Vec<i32>> {
        self.numbers
            .chunks(self.size)
            .map(|c| c.to_vec())
            .collect_vec()
    }

    fn columns(&self) -> Vec<Vec<i32>> {
        transpose(self.rows())
    }

    // assuming the last number is the winning number
    fn calculate_score(&self, drawn_numbers: &[&i32]) -> i32 {
        let result = self
            .numbers
            .iter()
            .filter(|num| !drawn_numbers.contains(num))
            .sum::<i32>();

        match drawn_numbers.last() {
            Some(x) => *x * result,
            None => 0,
        }
    }
}

fn task_1(data: &BingoInput) -> String {
    let mut winning_board: Option<&BingoBoard> = None;
    let mut attempted_numbers = vec![];
    for number in &data.drawn_numbers {
        attempted_numbers.push(number);
        let win = data
            .boards
            .iter()
            .find(|board| board.is_winning(&attempted_numbers));

        if let Some(board) = win {
            winning_board = Some(board);
            break;
        }
    }
    match winning_board {
        Some(board) => board.calculate_score(&attempted_numbers).to_string(),
        None => String::from("did not find any winning boards"),
    }
}

fn task_2(data: &BingoInput) -> String {
    let mut first_losing_board: Option<&BingoBoard> = None;
    let mut attempted_numbers = data.drawn_numbers.iter().collect_vec();
    let mut last_popped: &i32 = &0;
    for _ in data.drawn_numbers.iter().rev().enumerate() {
        let lose = data
            .boards
            .iter()
            .find(|board| !board.is_winning(&attempted_numbers));

        match lose {
            Some(board) => {
                first_losing_board = Some(board);
                attempted_numbers.push(last_popped);
                break;
            }
            None => {
                last_popped = attempted_numbers.pop().unwrap();
            }
        }
    }
    match first_losing_board {
        Some(board) => board.calculate_score(&attempted_numbers).to_string(),
        None => String::from("did not the last board"),
    }
}

#[test]
fn test() {
    let test_input = String::from(
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n
\n
22 13 17 11  0\n
 8  2 23  4 24\n
21  9 14 16  7\n
 6 10  3 18  5\n
 1 12 20 15 19\n
\n
 3 15  0  2 22\n
 9 18 13 17  5\n
19  8  7 25 23\n
20 11 10 24  4\n
14 21 16 12  6\n
\n
14 21 17 24  4\n
10 16 15  9 19\n
18  8 23 26 20\n
22 11 13  6  5\n
 2  0 12  3  7\n",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "4512");
    assert_eq!(task_2(&parsed), "1924");
}

fn parse(input: &str) -> BingoInput {
    let lines = input.lines().collect_vec();

    let drawn_numbers = lines[0]
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();

    let board_size = 5;

    let boards: Vec<BingoBoard> = lines
        .iter()
        .filter(|&line| !line.contains(','))
        .filter(|&line| line != &"\n")
        .join(" ")
        .split(' ')
        .filter(|&line| !line.is_empty())
        .map(|num| num.parse::<i32>().unwrap())
        .collect_vec()
        .chunks(board_size * board_size)
        .map(|x| x.to_vec())
        .map(|item| BingoBoard {
            numbers: item,
            size: board_size,
        })
        .collect_vec();

    BingoInput {
        boards,
        drawn_numbers,
    }
}
