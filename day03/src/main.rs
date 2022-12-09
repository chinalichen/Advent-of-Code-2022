use std::fs;

#[test]
fn test_priority() {
    let p = priority("NnjjRlnWNSWWbGwccbcchfPfTvfjfTBBpvmdMjTfvB");
    assert_eq!(p as u8, b'j' - b'a' + 1);
}

#[test]
fn test_badge() {
    let b = badge(&vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
    ]);
    assert_eq!(b, item_value(b'r'));
}

fn item_value(item: u8) -> u32 {
    // upper case
    if item < b'a' {
        return (item - b'A' + 27) as u32;
    }

    // lower case
    return (item - b'a' + 1) as u32;
}

fn badge(rss: &Vec<&str>) -> u32 {
    let mut array_set = vec![0; 100]; // ascii 0:A 1:B ...

    for (i, rs) in rss.iter().enumerate() {
        for item in rs.as_bytes() {
            let idx = item - b'A';
            if array_set[idx as usize] == i {
                array_set[idx as usize] = i + 1;
                if i == 2 {
                    return item_value(*item);
                }
            }
        }
    }
    return 0;
}

fn priority(rucksack: &str) -> u32 {
    let mut array_set = vec![0; 100]; // ascii 0:A 1:B ...

    let len = rucksack.len();

    if len == 0 {
        return 0;
    }
    if len % 2 != 0 {
        panic!("even length.");
    }
    let bytes = rucksack.as_bytes();

    // cache the first compartment
    for i in 0..len / 2 {
        array_set[(bytes[i] - b'A') as usize] = 1;
    }

    // find in ths second compartment
    for i in len / 2..len {
        let item = bytes[i];

        if array_set[(item - b'A') as usize] == 0 {
            continue;
        }
        return item_value(item);
    }

    return 0;
}

fn main() {
    let input_text = fs::read_to_string("day03/input.txt").unwrap();
    let rucksacks = input_text.split("\n");

    let mut sum: u32 = 0;
    // for rucksack in rucksacks {
    //     sum += priority(rucksack);
    // }

    let mut rss = vec!["a"; 3];
    for (i, rucksack) in rucksacks.into_iter().enumerate() {
        rss[i % 3] = rucksack;
        if i % 3 == 2 {
            sum += badge(&rss);
        }
    }

    println!("first puzzle {}", sum);
}
