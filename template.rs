use crate::solve;

pub fn run() {
    let day_number = 0;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(_data: &ParsedInput) -> String {
    todo!()
}

fn task_2(_data: &ParsedInput) -> String {
    todo!()
}
#[derive(Debug)]
struct ParsedInput {}
fn parse(input: &str) -> ParsedInput {
    ParsedInput {}
}

#[test]
fn test() {
    let test_input = String::from("");
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "");
    assert_eq!(task_2(&parsed), "");
}
