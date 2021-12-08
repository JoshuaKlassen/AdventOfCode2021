use std::io::{self, Read};

struct SubPosition {
    depth: i32,
    horizontal: i32,
}

impl SubPosition {
    fn add_position(&mut self, sub_pos: SubPosition) {
        self.depth += sub_pos.depth;
        self.horizontal += sub_pos.horizontal;
    }

    fn empty() -> SubPosition {
        SubPosition {
            depth: 0,
            horizontal: 0,
        }
    }
}

struct SubPositionWithAim {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl SubPositionWithAim {
    fn empty() -> SubPositionWithAim {
        SubPositionWithAim {
            depth: 0,
            horizontal: 0,
            aim: 0
        }
    }
}

fn part_one(input: &str) -> i32 {
    let mut sub_pos = SubPosition::empty();

    for line in input.lines() {
        let update_pos = parse_command(line);

        sub_pos.add_position(update_pos);
    }

    sub_pos.depth * sub_pos.horizontal
}

fn parse_command(cmd: &str) -> SubPosition {
    let mut depth = 0;
    let mut horizontal = 0;

    let mut tokens = cmd.split_whitespace();
    let action = tokens.next().unwrap();
    let adjustment = tokens.next().expect("Could not read adjustmnet");
    let adjustment = adjustment.parse::<i32>().unwrap();

    match action.to_lowercase().as_str() {
        "forward" => horizontal = adjustment,
        "up" => depth = -adjustment,
        "down" => depth = adjustment,
        _ => panic!("Could not parse command: {}", cmd)
    }

    SubPosition {
        depth: depth,
        horizontal: horizontal,
    }
}

fn parse_command_into_position(cmd: &str, sub_pos: &mut SubPositionWithAim) {
    let tokens:Vec<&str> = cmd.split_whitespace().collect();
    let action = tokens[0].trim();
    let adjustment = tokens[1].parse::<i32>().unwrap();

    match action.to_lowercase().as_str() {
        "forward" => {sub_pos.horizontal += adjustment; sub_pos.depth += sub_pos.aim * adjustment},
        "up" => sub_pos.aim -= adjustment,
        "down" => sub_pos.aim += adjustment,
        _ => panic!("Could not parse command: {}", cmd)
    }
}

fn part_two(input: &str) -> i32 {
    let mut sub_pos = SubPositionWithAim::empty();

    for line in input.lines() {
        parse_command_into_position(line, &mut sub_pos);
    }

    sub_pos.depth * sub_pos.horizontal
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(& mut buffer).expect("Failed to read input");

    let part_one_result = part_one(&buffer);
    println!("Day 2 part one: {}", part_one_result);

    let part_two_result = part_two(&buffer);
    println!("Day 2 part two: {}", part_two_result);
}

#[test]
fn test_add_position() {
    let mut x = SubPosition {depth: 10, horizontal: 5};
    let y = SubPosition {depth: -3, horizontal: 2};
    x.add_position(y);

    assert_eq!(7, x.depth);
    assert_eq!(7, x.horizontal);
}

#[test]
fn test_parse_command() {
    let sub_pos = parse_command("forward 5");
    assert_eq!(0, sub_pos.depth);
    assert_eq!(5, sub_pos.horizontal);

    let sub_pos = parse_command("up 3");
    assert_eq!(-3, sub_pos.depth);
    assert_eq!(0, sub_pos.horizontal);

    let sub_pos = parse_command("down 7");
    assert_eq!(7, sub_pos.depth);
    assert_eq!(0, sub_pos.horizontal);
}

#[test]
fn test_part_one() {
    let result = part_one("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n");
    assert_eq!(150, result);
}

#[test]
fn test_parse_command_into_position() {
    let mut sub_pos = SubPositionWithAim::empty();
    parse_command_into_position("forward 5\n", &mut sub_pos);
    parse_command_into_position("down 5\n", &mut sub_pos);
    parse_command_into_position("forward 8\n", &mut sub_pos);
    parse_command_into_position("up 3\n", &mut sub_pos);

    assert_eq!(2, sub_pos.aim);
    assert_eq!(13, sub_pos.horizontal);
    assert_eq!(40, sub_pos.depth);
}

#[test]
fn test_part_two() {
    let result = part_two("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n");
    assert_eq!(900, result);
}