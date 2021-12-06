use crate::solve;

pub fn run() {
    let day_number = 0;

    solve!(&day_number, parse, task_1, task_2);
}

fn parse(input: &String) -> Any {}

fn task_1(data: &Any) -> String {
    String::from("TODO")
}

fn task_2(data: &Any) -> String {
    String::from("TODO")
}

#[test]
fn test() {
    let test_input = String::from("");
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "");
    assert_eq!(task_2(&parsed), "");
}
