use std::fs;

fn instruction_delta(ins: &str) -> i32 {
    let delta = ins
        .split(" ")
        .skip(1)
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    return delta;
}

fn signal_strengths(input_str: String) -> i32 {
    let milestones = vec![20, 60, 100, 140, 180, 220, -1];
    let mut milestone_idx = 0;
    let mut register_x = 1;
    let mut sum_signal_strengths = 0;
    let mut cycle_count = 1;
    for instruction in input_str.split("\n") {
        let ms = milestones[milestone_idx];
        match instruction {
            "noop" => {
                cycle_count += 1;
                if cycle_count == ms {
                    sum_signal_strengths += ms * register_x;
                    milestone_idx += 1;
                }
            }
            add_instruction => {
                cycle_count += 1;
                if cycle_count == ms {
                    sum_signal_strengths += ms * register_x;
                    milestone_idx += 1;
                }

                let delta = instruction_delta(add_instruction);
                register_x += delta;
                cycle_count += 1;
                if cycle_count == ms {
                    sum_signal_strengths += ms * register_x;
                    milestone_idx += 1;
                }
            }
        }
    }
    return sum_signal_strengths;
}

#[test]
fn test_signal_strengths() {
    let str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(signal_strengths(str.to_string()), 13140);
}

fn produce_image(input_str: String) {
    let mut screen = vec![b'.'; 300];
    let mut register_x: i32 = 1;
    let mut cycle_count = 1;

    let mut draw = |cycle: i32, reg: i32| -> () {
        let mut h = cycle % 40;
        if h == 0 {
            h = 40;
        }
        if h >= reg && h <= reg + 2 {
            screen[cycle as usize] = b'#';
        }
    };

    for instruction in input_str.split("\n") {
        match instruction {
            "noop" => {
                draw(cycle_count, register_x);
                cycle_count += 1;
            }
            add_instruction => {
                draw(cycle_count, register_x);
                cycle_count += 1;

                draw(cycle_count, register_x);
                let delta = instruction_delta(add_instruction);
                register_x += delta;
                cycle_count += 1;
            }
        }
    }

    for i in 1..241 {
        print!("{}", screen[i] as char);
        if i % 40 == 0 {
            print!("\n");
        }
    }
}

#[test]
fn test_produce_image() {
    let str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    produce_image(str.to_string())
}

fn main() {
    let input_str = fs::read_to_string("day10/input.txt").unwrap();
    // let strengths = signal_strengths(input_str.to_string());
    // println!("first puzzle {}", strengths);
    produce_image(input_str);
}
