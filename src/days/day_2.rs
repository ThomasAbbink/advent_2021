use std::fs;

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
    let data = get_data();
    task_1(&data);
    task_2(&data);
}

fn task_1(commands: &Vec<Command>) {
    let (forward, depth) =
        commands.iter().fold((0, 0), |(forward, depth), command| {
            match command.direction {
                Direction::Up => (forward, depth - command.amount),
                Direction::Down => (forward, depth + command.amount),
                Direction::Forward => (forward + command.amount, depth),
                _ => (forward, depth),
            }
        });

    let final_depth: i32 = commands
        .iter()
        .map(|command| match &command.direction {
            Direction::Up => -command.amount,
            Direction::Down => command.amount,
            _ => 0,
        })
        .sum();

    let old_forward: i32 = commands
        .iter()
        .map(|command| match &command.direction {
            Direction::Forward => command.amount,
            _ => 0,
        })
        .sum();

    println!("depth: {} {}", final_depth, depth);
    println!("forwards: {} {}", old_forward, forward);
    println!("answer: {}", final_depth * forward);
}

fn task_2(commands: &Vec<Command>) {
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
    println!("{}", forward * depth)
}

fn get_data() -> Vec<Command> {
    let path = "./src/days/day_2_input.txt";
    let file = fs::read_to_string(path).expect("could not open file");
    file.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
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
