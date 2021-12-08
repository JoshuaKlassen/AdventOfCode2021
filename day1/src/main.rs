use std::{io::{self, Read}};

fn part_one(input: &str) -> i32 {
    let mut increases = 0;
    let mut last_value = 0;

    for line in input.lines() {
        let value = line.trim().parse::<i32>().unwrap();

        if last_value == 0 {
            last_value = value;
        }

        if value > last_value {
            increases += 1;
        }

        last_value = value;
    }

    increases
}

fn part_two(input: &str) -> i32 {
    let mut increases = 0;
    let mut last_sum = 0;
    
    let values: Vec<i32> = input
        .lines()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let mut index = 2;
    while index < values.len() {
        let sum = values[index - 2] + values[index - 1] + values[index];
    
        if last_sum == 0 {
            last_sum = sum;
        }

        if sum > last_sum {
            increases += 1;
        }

        last_sum = sum;
        index += 1;
    }

    increases
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let increasing_measurements = part_one(&buffer);
    println!("Day one part one: {}", increasing_measurements);

    let increasing_measurements = part_two(&buffer);
    println!("Day two part two: {}", increasing_measurements);
}

#[test]
fn test_part_one() {
    assert_eq!(7, part_one("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"))
}

#[test]
fn test_part_two() {
    assert_eq!(5, part_two("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"));
}