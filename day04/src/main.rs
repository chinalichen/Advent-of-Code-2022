use std::fs;

fn parse_section(section: &str) -> (i32, i32) {
    let mut iter = section.split("-").map(|n| n.parse::<i32>().unwrap());
    return (iter.next().unwrap(), iter.next().unwrap());
}

fn parse_pair(pair: &str) -> (i32, i32, i32, i32) {
    let mut iter = pair.split(",").map(parse_section);
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    return (first.0, first.1, second.0, second.1);
}

fn is_contain(pair: &str) -> bool {
    let (first_start, first_end, second_start, second_end) = parse_pair(pair);
    if first_start >= second_start && first_end <= second_end {
        return true;
    }
    if first_start <= second_start && first_end >= second_end {
        return true;
    }
    return false;
}

#[test]
fn test_is_contain() {
    let result = is_contain("82-82,8-83");
    assert_eq!(result, true);
}

fn in_section(start: i32, end: i32, point: i32) -> bool {
    return start <= point && end >= point;
}

fn is_overlap(pair: &str) -> bool {
    let (first_start, first_end, second_start, second_end) = parse_pair(pair);
    if in_section(first_start, first_end, second_start) {
        return true;
    }
    if in_section(first_start, first_end, second_end) {
        return true;
    }
    if in_section(second_start, second_end, first_start) {
        return true;
    }
    if in_section(second_start, second_end, first_end) {
        return true;
    }
    return false;
}
#[test]
fn test_is_overlap() {
    let cases = vec![
        ("2-4,6-8", false),
        ("2-3,4-5", false),
        ("5-7,7-9", true),
        ("2-8,3-7", true),
        ("6-6,4-6", true),
        ("2-6,4-8", true),
        ("6-8,2-4", false),
        ("4-5,2-3", false),
        ("7-9,5-7", true),
        ("3-7,2-8", true),
        ("4-6,6-6", true),
        ("4-8,2-6", true),
    ];
    for case in cases {
        let result = is_overlap(case.0);
        assert_eq!(result, case.1);
    }
}

fn main() {
    let input_text = fs::read_to_string("day04/input.txt").unwrap();
    let pairs = input_text.split("\n");

    let mut count: u32 = 0;
    for pair in pairs {
        if pair.len() == 0 {
            break;
        }
        // if is_contain(pair) {
        //     count += 1;
        // }
        if is_overlap(pair) {
            count += 1;
        }
    }

    println!("first puzzle {}", count);
}
