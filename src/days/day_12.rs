use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 12;

    solve!(&day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let start: Vec<Vec<String>> = vec![vec![String::from("start")]];
    let paths = calculate_paths(&data.nodes, start, 0);
    paths.len().to_string()
}

fn calculate_paths(
    nodes: &HashMap<String, Node>,
    paths: Vec<Vec<String>>,
    small_node_duplicates_allowed: usize,
) -> Vec<Vec<String>> {
    let not_done = paths
        .iter()
        .filter(|p| *p.last().unwrap() != String::from("end"))
        .collect_vec();

    if not_done.is_empty() {
        paths
    } else {
        let mut new_paths: Vec<Vec<String>> = vec![];

        // loop over all paths
        for path in paths {
            let current_node = nodes.get(&*path.last().unwrap()).unwrap();

            // if the current node is 'end' skip, but keep the path
            if let NodeType::End = current_node.node_type {
                new_paths.push(path.to_vec());
                continue;
            }

            // loop over each connected node
            for connected_node in current_node
                .connections
                .iter()
                .map(|c| nodes.get(c).unwrap())
            {
                // if the connected node is small and the connected node already exists, this path is a dead end.
                if connected_node.node_type == NodeType::Small {
                    // count the duplicates of small nodes
                    let mut copy = path.clone();
                    copy.push(connected_node.id.to_string());
                    let small_nodes = copy
                        .iter()
                        .map(|s| nodes.get(s).unwrap())
                        .filter(|n| n.node_type == NodeType::Small)
                        .collect_vec();
                    let uniques = small_nodes.iter().unique().count();
                    if (small_nodes.len() - uniques) > small_node_duplicates_allowed {
                        continue;
                    }
                }
                // we can't go back to the start
                if connected_node.id.as_str() != "start" {
                    let mut new_path = path.to_vec();
                    new_path.push(connected_node.id.to_string());
                    new_paths.push(new_path);
                }
            }
        }

        calculate_paths(nodes, new_paths, small_node_duplicates_allowed)
    }
}

fn task_2(data: &ParsedInput) -> String {
    let start: Vec<Vec<String>> = vec![vec![String::from("start")]];
    let paths = calculate_paths(&data.nodes, start, 1);
    paths.len().to_string()
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum NodeType {
    Start,
    End,
    Big,
    Small,
}

impl FromStr for NodeType {
    type Err = ();
    fn from_str(input: &str) -> Result<NodeType, Self::Err> {
        match input {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            _ => {
                let s = String::from(input);
                let c = *s.chars().collect_vec().first().unwrap();
                let is_uppercase = c.is_ascii_uppercase();
                if is_uppercase {
                    Ok(Self::Big)
                } else {
                    Ok(Self::Small)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    node_type: NodeType,
    id: String,
    connections: Vec<String>,
}

impl Node {
    fn from(id: &str, connections: Vec<&str>) -> Node {
        Node {
            id: String::from(id),
            connections: connections.iter().map(|s| String::from(*s)).collect_vec(),
            node_type: NodeType::from_str(id).unwrap(),
        }
    }
}

#[derive(Debug)]
struct ParsedInput {
    nodes: HashMap<String, Node>,
}

fn parse(input: &str) -> ParsedInput {
    let nodes = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (a, b) = line.split('-').collect_tuple().unwrap();
            vec![(a, b), (b, a)]
        })
        .flatten()
        .fold(
            HashMap::default(),
            |mut hm: HashMap<&str, Vec<&str>>, (a, b)| {
                let connections = hm.entry(a).or_insert_with(Vec::new);
                connections.push(b);
                hm
            },
        )
        .iter()
        .map(|(id, connections)| (String::from(*id), Node::from(id, connections.to_vec())))
        .collect::<HashMap<String, Node>>();
    ParsedInput { nodes }
}

#[test]
fn test() {
    let test_input = String::from(
        "\nfs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW",
    );
    let test_input_2 = String::from("\nstart-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end");
    let parsed = parse(&test_input);
    let parsed_2 = parse(&test_input_2);
    assert_eq!(task_1(&parsed), "226");
    assert_eq!(task_1(&parsed_2), "10");
    assert_eq!(task_2(&parsed_2), "36");
    assert_eq!(task_2(&parsed), "3509");
}
