use std::fs;
use std::str::Split;

fn parse_direction(direction: &str) -> (i32, i32, i32) {
    let mut parts = direction.split(" ").enumerate();

    parts.next();
    let count = parts.next().unwrap().1.parse::<i32>().unwrap();
    parts.next();
    let from = parts.next().unwrap().1.parse::<i32>().unwrap();
    parts.next();
    let to = parts.next().unwrap().1.parse::<i32>().unwrap();

    return (count, from, to);
}
#[test]
fn test_is_overlap() {
    let cases = vec![
        ((3, 4, 3), "move 3 from 4 to 3"),
        ((3, 8, 6), "move 3 from 8 to 6"),
        ((2, 3, 8), "move 2 from 3 to 8"),
    ];
    for case in cases {
        let result = parse_direction(case.1);
        assert_eq!(result, case.0);
    }
}

fn crate_mover_9000(mut init_stacks: Vec<Vec<char>>, directions: Split<&str>) -> String {
    for direction in directions {
        let (count, from, to) = parse_direction(direction);
        for _ in 0..count {
            let cargo = init_stacks[from as usize - 1].pop();
            match cargo {
                None => {
                    panic!("wront diection: {}\n", direction)
                }
                Some(cargo) => {
                    init_stacks[to as usize - 1].push(cargo);
                }
            }
        }
    }
    return init_stacks
        .into_iter()
        .map(|vc| vc[vc.len() - 1])
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>();
}

fn crate_mover_9001(mut init_stacks: Vec<Vec<char>>, directions: Split<&str>) -> String {
    for direction in directions {
        let (count, from, to) = parse_direction(direction);
        let mut temp = vec![];
        for _ in 0..count {
            let cargo = init_stacks[from as usize - 1].pop().unwrap();
            temp.push(cargo);
        }
        for _ in 0..count {
            init_stacks[to as usize - 1].push(temp.pop().unwrap());
        }
    }
    return init_stacks
        .into_iter()
        .map(|vc| vc[vc.len() - 1])
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>();
}

fn main() {
    //      [W]         [J]     [J]
    //      [V]     [F] [F] [S] [S]
    //      [S] [M] [R] [W] [M] [C]
    //      [M] [G] [W] [S] [F] [G]     [C]
    //  [W] [P] [S] [M] [H] [N] [F]     [L]
    //  [R] [H] [T] [D] [L] [D] [D] [B] [W]
    //  [T] [C] [L] [H] [Q] [J] [B] [T] [N]
    //  [G] [G] [C] [J] [P] [P] [Z] [R] [H]
    //   1   2   3   4   5   6   7   8   9
    let mut init_stacks = vec![
        vec!['G', 'T', 'R', 'W'],
        vec!['G', 'C', 'H', 'P', 'M', 'S', 'V', 'W'],
        vec!['C', 'L', 'T', 'S', 'G', 'M'],
        vec!['J', 'H', 'D', 'M', 'W', 'R', 'F'],
        vec!['P', 'Q', 'L', 'H', 'S', 'W', 'F', 'J'],
        vec!['P', 'J', 'D', 'N', 'F', 'M', 'S'],
        vec!['Z', 'B', 'D', 'F', 'G', 'C', 'S', 'J'],
        vec!['R', 'T', 'B'],
        vec!['H', 'N', 'W', 'L', 'C'],
    ];
    let directions_str = fs::read_to_string("day05/input.txt").unwrap();
    let directions = directions_str.split("\n");

    // println!(
    //     "first puzzle {:?}",
    //     crate_mover_9000(init_stacks, directions),
    // );

    println!(
        "first puzzle {:?}",
        crate_mover_9001(init_stacks, directions),
    );
}
