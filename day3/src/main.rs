use std::{io::{self, Read}, collections::HashMap};

fn part_one(input: &str) -> i32 {
    let mut data = HashMap::<usize, (i32, i32)>::new();

    for line in input.lines() {
        let bit_string = line.trim();

        for (i, bit) in bit_string.chars().into_iter().enumerate() {
            let counts = data.entry(i).or_insert((0,0));

            match bit {
                '1' => *counts = (counts.0, counts.1 + 1),
                '0' => *counts = (counts.0 + 1, counts.1),
                _ => panic!("Unable to process bit: {}", bit)
            }
        }
    }

    let mut bit_string_one = String::new();
    let mut bit_string_two = String::new();

    let mut index = 0;
    while index < data.keys().len() {
        let counts = data.get(&index).unwrap();

        if counts.0 > counts.1 {
            bit_string_one.push_str("0");
            bit_string_two.push_str("1");
        } else {
            bit_string_one.push_str("1");
            bit_string_two.push_str("0");
        }

        index +=1;
    }

    let gamma = convert_bit_string_to_int(&bit_string_one);
    let epsilon = convert_bit_string_to_int(&bit_string_two);

    gamma * epsilon
}

fn convert_bit_string_to_int(bit_string: &str) -> i32 {
    let mut result = 0;

    let mut power = 0;
    for bit in bit_string.chars().rev() {
        if bit == '1' {
            result += i32::pow(2, power);
        }
        power += 1;
    }

    result
}

fn part_two(input: &str) -> i32 {
    let mut oxygen_values = Vec::<&str>::new();
    let mut scrubber_values = Vec::<&str>::new();

    for line in input.lines() {
        let bit_string = line.trim();
        oxygen_values.push(&bit_string);
        scrubber_values.push(&bit_string);
    }

    let oxygen_int = compute_oxgyen_value(oxygen_values);
    let scrubber_int = compute_scrubber_value(scrubber_values);

    oxygen_int * scrubber_int
}

fn compute_oxgyen_value(oxgyen_bit_strings: Vec<&str>) -> i32 {
    let mut oxygen_values = oxgyen_bit_strings.clone();
    let mut bit_index = 0;
    while bit_index < oxygen_values.first().unwrap().len() {
        let mut ones = 0;
        let mut zeros = 0;

        for bit_string in &oxygen_values {
            let bit = bit_string.chars().nth(bit_index).unwrap();
            match bit {
                '1' => ones += 1,
                '0' => zeros += 1,
                _ => panic!("Could not parse bit: {}", bit)
            }
        }

        if ones >= zeros {
            oxygen_values = remove_values_for_common_bit(bit_index, '1', oxygen_values);
        } else {
            oxygen_values = remove_values_for_common_bit(bit_index, '0', oxygen_values);
        }

        if oxygen_values.len() == 1 {
            break;
        }

        bit_index += 1;
    }

    let oxygen_bit_string = oxygen_values.first().unwrap();

    convert_bit_string_to_int(oxygen_bit_string)
}

fn compute_scrubber_value(srubber_bit_strings: Vec<&str>) -> i32 {
    let mut scrubber_values = srubber_bit_strings.clone();
    let mut bit_index = 0;
    while bit_index < scrubber_values.first().unwrap().len() {
        let mut ones = 0;
        let mut zeros = 0;

        for bit_string in &scrubber_values {
            let bit = bit_string.chars().nth(bit_index).unwrap();
            match bit {
                '1' => ones += 1,
                '0' => zeros += 1,
                _ => panic!("Could not parse bit: {}", bit)
            }
        }

        if ones >= zeros {
            scrubber_values = remove_values_for_common_bit(bit_index, '0', scrubber_values);
        } else {
            scrubber_values = remove_values_for_common_bit(bit_index, '1', scrubber_values);
        }

        if scrubber_values.len() == 1 {
            break;
        }

        bit_index += 1;
    }

    let scrubber_bit_string = scrubber_values.first().unwrap();

    convert_bit_string_to_int(scrubber_bit_string)
}


fn remove_values_for_common_bit(index: usize, bit: char, values: Vec<&str>) -> Vec<&str>{
    let mut valid_values = Vec::<&str>::new();
    for value in values.clone() {
        if value.chars().nth(index).unwrap() == bit {
            valid_values.push(value);
        }
    }

    valid_values
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 3 part one: {}", result);

    let result = part_two(&buffer);
    println!("Day 3 part two: {}", result);
}

#[test]
fn test_part_one() {
    assert_eq!(198, part_one("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"));
}

#[test]
fn test_convert_bit_string_to_int() {
    assert_eq!(22, convert_bit_string_to_int("10110"));
    assert_eq!(9, convert_bit_string_to_int("01001"));
}

#[test]
fn test_remove_values_for_common_bit() {
    let bit_strings = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"];
    let result = remove_values_for_common_bit(0, '1', bit_strings);
    assert_eq!(7, result.len());
}

#[test]
fn test_compute_oxygen_value() {
    let bit_strings = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"];
    assert_eq!(23, compute_oxgyen_value(bit_strings));
}

#[test]
fn test_compute_scrubber_value() {
    let bit_strings = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"];
    assert_eq!(10, compute_scrubber_value(bit_strings));
}

#[test]
fn test_part_two() {
    assert_eq!(230, part_two("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"));
}