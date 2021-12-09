use crate::solve;

#[derive(Debug)]
struct Command {
    direction: Direction,
    amount: i32,
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
    Error,
}

pub fn run() {
    solve!(&2, parse, task_1, task_2);
}

fn task_1(commands: &[Command]) -> String {
    let (forward, depth) =
        commands.iter().fold((0, 0), |(forward, depth), command| {
            match command.direction {
                Direction::Up => (forward, depth - command.amount),
                Direction::Down => (forward, depth + command.amount),
                Direction::Forward => (forward + command.amount, depth),
                _ => (forward, depth),
            }
        });
    let answer = forward * depth;
    answer.to_string()
}

fn task_2(commands: &[Command]) -> String {
    let mut aim = 0;
    let mut depth = 0;
    let mut forward = 0;

    for command in commands {
        match command.direction {
            Direction::Down => aim += command.amount,
            Direction::Up => aim -= command.amount,
            Direction::Forward => {
                forward += command.amount;
                depth += command.amount * aim;
            }
            _ => (),
        }
    }
    let answer = forward * depth;
    answer.to_string()
}

fn parse(data: &str) -> Vec<Command> {
    data.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let direction = match parts[0] {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => Direction::Error,
            };
            let amount = parts[1].parse::<i32>().unwrap();
            Command { direction, amount }
        })
        .collect()
}
