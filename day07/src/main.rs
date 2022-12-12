use std::fs::{self};

type NodeId = usize;

#[derive(Debug)]
enum NodeType {
    Dir,
    File,
}

#[derive(Debug)]
struct Node {
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    name: String,
    size: u32,
    kind: NodeType,
}

fn travel_node_size(nodes: &Vec<Node>, root_node_id: NodeId, result: &mut Vec<u32>) -> u32 {
    let node = &nodes[root_node_id];
    match node.kind {
        NodeType::File => {
            return node.size;
        }
        NodeType::Dir => {
            let size = node
                .children
                .iter()
                .map(|id| travel_node_size(nodes, *id, result))
                .fold(0, |acc, s| acc + s);
            result[root_node_id] = size;
            return size;
        }
    }
}

fn main() {
    let input = fs::read_to_string("day07/input.txt").unwrap();
    let lines = input.split("\n");

    let mut nodes = vec![Node {
        parent: None,
        children: vec![],
        name: "/".to_string(),
        size: 0,
        kind: NodeType::Dir,
    }];
    let mut curr_pos = 0;

    for line in lines {
        if line.starts_with('$') {
            // command
            let mut parts = line.split(" ");
            parts.next();
            let command = parts.next().unwrap();
            match command {
                "cd" => {
                    let directory_name = parts.next().unwrap();
                    match directory_name {
                        "/" => {
                            curr_pos = 0;
                        }
                        ".." => {
                            let curr = &nodes[curr_pos];
                            curr_pos = curr.parent.expect("root has no parent.");
                        }
                        dir_name => {
                            let curr = &nodes[curr_pos];
                            let child_idx = curr
                                .children
                                .iter()
                                .position(|n| nodes[*n].name == dir_name)
                                .unwrap();
                            curr_pos = curr.children[child_idx];
                        }
                    }
                }
                "ls" => {}
                _ => {}
            };
        } else {
            // output
            let mut parts = line.split(" ");
            let dir_or_file = parts.next().unwrap();
            match dir_or_file {
                "dir" => {
                    // TODO: check duplicate.
                    let dir = Node {
                        parent: Some(curr_pos),
                        children: vec![],
                        name: parts.next().unwrap().to_string(),
                        size: 0,
                        kind: NodeType::Dir,
                    };
                    let node_id = nodes.len();
                    nodes[curr_pos].children.push(node_id);
                    nodes.push(dir);
                }
                size_str => {
                    let file = Node {
                        parent: Some(curr_pos),
                        children: vec![],
                        name: parts.next().unwrap().to_string(),
                        size: size_str.parse::<u32>().unwrap(),
                        kind: NodeType::File,
                    };
                    let node_id = nodes.len();
                    nodes[curr_pos].children.push(node_id);
                    nodes.push(file);
                }
            }
        }
    }

    let result = &mut vec![0; 1000];
    travel_node_size(&nodes, 0, result);

    println!(
        "first puzzle {:?}",
        result
            .iter()
            .filter(|v| **v < 100000)
            .fold(0, |acc, s| acc + s)
    );

    result.sort();

    println!(
        "second puzzle {:?}",
        result
            .iter()
            .filter(|v| **v > result.last().unwrap() + 30000000 - 70000000)
            .next()
            .unwrap()
    );
}
