use std::{collections::HashSet, fs};

fn is_start(a: u8, b: u8, c: u8, d: u8) -> bool {
    return a != b && a != c && a != d && b != c && b != d && c != d;
}

fn start_of_packet(packet: &[u8]) -> Result<usize, &str> {
    for i in 0..packet.len() - 4 {
        if is_start(packet[i], packet[i + 1], packet[i + 2], packet[i + 3]) {
            return Ok(i + 4);
        }
    }

    return Err("not found");
}

#[test]
fn test_start_of_packet() {
    assert_eq!(
        start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()).unwrap(),
        5
    );
    assert_eq!(
        start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()).unwrap(),
        11
    );
}

fn is_start_of_data(slice: &[u8]) -> bool {
    let mut set = HashSet::new();
    for char in slice {
        if set.contains(char) {
            return false;
        }
        set.insert(char);
    }
    return true;
}

fn start_data_of_packet(packet: &[u8]) -> Result<usize, &str> {
    for i in 0..packet.len() - 14 {
        if is_start_of_data(&packet[i..i + 14]) {
            return Ok(i + 14);
        }
    }
    return Err("not found");
}

#[test]
fn test_start_data_of_packet() {
    assert_eq!(
        start_data_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes()).unwrap(),
        19
    );
    assert_eq!(
        start_data_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()).unwrap(),
        23
    );
}

fn main() {
    let chars = fs::read_to_string("day06/input.txt").unwrap();
    println!(
        "first puzzle result: {}\n",
        start_of_packet(chars.as_bytes()).unwrap()
    );
    println!(
        "second puzzle result: {}\n",
        start_data_of_packet(chars.as_bytes()).unwrap()
    );
}