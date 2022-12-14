use std::fs;

#[derive(Debug, Copy, Clone)]
struct Point {
    row: i32,
    col: i32,
}

fn single_direction(v: i32) -> i32 {
    if v == 0 {
        return 0;
    } else if v > 0 {
        return 1;
    } else {
        return -1;
    }
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    return (p1.row - p2.row) * (p1.row - p2.row) + (p1.col - p2.col) * (p1.col - p2.col);
}

fn should_move(distance: i32) -> bool {
    return distance > 2;
}

fn move_tail(head: &Point, tail: &Point) -> Option<Point> {
    if !should_move(distance(head, tail)) {
        return None;
    }
    let delta_row: i32 = head.row - tail.row;
    let delta_col: i32 = head.col - tail.col;
    return Some(Point {
        row: tail.row + single_direction(delta_row),
        col: tail.col + single_direction(delta_col),
    });
}

fn visited(input_str: String) -> i32 {
    let size = 1000;
    let mut map = vec![vec![0; size]; size];

    let center = (size / 2) as i32;
    let mut head = Point {
        row: center,
        col: center,
    };
    let mut tail = Point {
        row: center,
        col: center,
    };

    map[tail.row as usize][tail.col as usize] = 1;

    for m in input_str.split("\n") {
        let mut parts = m.split(" ");
        let dir = parts.next().unwrap();
        let count = parts.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..count {
            match dir {
                "U" => {
                    head.row -= 1;
                }
                "D" => {
                    head.row += 1;
                }
                "L" => {
                    head.col -= 1;
                }
                "R" => {
                    head.col += 1;
                }
                _ => {}
            }
            let res = move_tail(&head, &tail);
            match res {
                None => {}
                new_tail => {
                    tail = new_tail.unwrap();
                    if tail.row < 0 || tail.col < 0 {
                        panic!("a haha");
                    }
                    map[tail.row as usize][tail.col as usize] = 1;
                }
            }
        }
    }

    let count = map
        .iter()
        .fold(0, |acc, vs| acc + vs.iter().fold(0, |ac, v| ac + v));

    return count;
}

#[test]
fn test_visited() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let res = visited(input.to_string());
    assert_eq!(res, 13);
}

fn second_visited(input_str: String) -> i32 {
    let size = 1000;
    let mut map = vec![vec![0; size]; size];

    let center = (size / 2) as i32;
    map[center as usize][center as usize] = 1;
    let mut knots = vec![
        Point {
            row: center,
            col: center
        };
        10
    ];

    for m in input_str.split("\n") {
        let mut parts = m.split(" ");
        let dir = parts.next().unwrap();
        let count = parts.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..count {
            match dir {
                "U" => {
                    knots[0].row -= 1;
                }
                "D" => {
                    knots[0].row += 1;
                }
                "L" => {
                    knots[0].col -= 1;
                }
                "R" => {
                    knots[0].col += 1;
                }
                _ => {}
            }
            for i in 1..10 {
                let res = move_tail(&knots[i - 1], &knots[i]);
                match res {
                    None => {}
                    new_tail => {
                        knots[i] = new_tail.unwrap();
                        if knots[i].row < 0 || knots[i].col < 0 {
                            panic!("a haha");
                        }
                    }
                }
            }
            map[knots[9].row as usize][knots[9].col as usize] = 1;
        }
    }

    let count = map
        .iter()
        .fold(0, |acc, vs| acc + vs.iter().fold(0, |ac, v| ac + v));

    return count;
}

#[test]
fn test_second_visited() {
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let res = second_visited(input.to_string());
    assert_eq!(res, 36);
}

fn main() {
    let moves_str = fs::read_to_string("day09/input.txt").unwrap();

    // println!("first puzzle {}", visited(moves_str));
    println!("second puzzle {}", second_visited(moves_str));
}
