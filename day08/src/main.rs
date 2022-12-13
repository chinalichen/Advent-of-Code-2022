use std::fs;

fn is_visible(map: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let curr_height = map[row][col];
    let mut up = true;
    let mut down = true;
    let mut left = true;
    let mut right = true;
    for r in 0..row {
        if map[r][col] >= curr_height {
            up = false;
        }
    }
    for r in row + 1..map.len() {
        if map[r][col] >= curr_height {
            down = false;
        }
    }
    for c in 0..col {
        if map[row][c] >= curr_height {
            left = false;
        }
    }
    for c in col + 1..map.len() {
        if map[row][c] >= curr_height {
            right = false;
        }
    }
    return up || down || left || right;
}

fn read_map(map_str: String, size: usize) -> Vec<Vec<u8>> {
    let lines = map_str.split("\n");

    // read map from input
    let mut map = vec![vec![b'0'; size]; size];
    for (row, line) in lines.enumerate() {
        for (col, height) in line.as_bytes().iter().enumerate() {
            map[row][col] = *height;
        }
    }
    return map;
}

fn count_visible(map_str: String, size: usize) -> usize {
    let map = read_map(map_str, size);

    let mut count = 0;
    // check all interior trees
    for r in 1..(size - 1) {
        for c in 1..(size - 1) {
            if is_visible(&map, r, c) {
                count += 1;
            }
        }
    }

    count += (size - 1) * 4;
    return count;
}

fn score(map: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let curr_height = map[row][col];
    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;
    for r in (0..row).rev() {
        if map[r][col] < curr_height {
            up += 1;
        } else {
            up += 1;
            break;
        }
    }
    for r in row + 1..map.len() {
        if map[r][col] < curr_height {
            down += 1;
        } else {
            down += 1;
            break;
        }
    }
    for c in (0..col).rev() {
        if map[row][c] < curr_height {
            left += 1;
        } else {
            left += 1;
            break;
        }
    }
    for c in col + 1..map.len() {
        if map[row][c] < curr_height {
            right += 1;
        } else {
            right += 1;
            break;
        }
    }
    return up * down * left * right;
}

fn highest_score(map_str: String, size: usize) -> usize {
    let map = read_map(map_str, size);

    let mut max = 0;
    // check all interior trees
    for r in 1..(size - 1) {
        for c in 1..(size - 1) {
            let s = score(&map, r, c);
            if s > max {
                max = s;
            }
        }
    }
    return max;
}

#[test]
fn test_count_visible() {
    let map_str = "30373\n25512\n65332\n33549\n35390";
    let count = count_visible(map_str.to_string(), 5);
    assert_eq!(count, 21);
}

#[test]
fn test_highest_score() {
    let map_str = "30373\n25512\n65332\n33549\n35390";
    assert_eq!(highest_score(map_str.to_string(), 5), 8);
}

fn main() {
    let map_str = fs::read_to_string("day08/input.txt").unwrap();
    // println!("first puzzle: {}", count_visible(map_str, 99));
    println!("second puzzle: {}", highest_score(map_str, 99));
}
