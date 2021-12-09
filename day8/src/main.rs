use std::{io::{self, Read}, collections::HashMap};

fn part_one(input: &str) -> i32 {
    let mut result = 0;

    for line in input.lines() {
        let mut tokens = line.split("|");
        tokens.next();
        let outputs = tokens.next().unwrap();
        let outputs: Vec<&str> = outputs.split_whitespace().collect();

        for output in outputs {
            let char_count = output.chars().count();

            match char_count {
                2 => result += 1,
                3 => result += 1,
                4 => result += 1,
                7 => result += 1,
                _ => {}
            }
        }

    }

    result
}

fn part_two(input: &str) -> i32 {
    let mut result = 0;

    for line in input.lines() {
        result += compute_display_values(&line);
    }

    result
}

fn compute_display_values(input: &str) -> i32 {
    let mut tokens = input.split("|");
    let signals = tokens.next().unwrap();

    let mut map = HashMap::<&str, i32>::new();
    let mut one = String::new();
    let mut four = String::new();
    let mut seven = String::new();
    let mut eight = String::new();

    let mut zero = String::new();
    let mut six = String::new();
    let mut nine = String::new();
    let mut six_digit_strings = Vec::<String>::new();

    let mut two = String::new();
    let mut three = String::new();
    let mut five = String::new();
    let mut five_digit_strings = Vec::<String>::new();

    for line in signals.split_whitespace() {
        let char_count = line.chars().count();

        match char_count {
            2 => one = sort_segment_input(line),
            3 => seven = sort_segment_input(line),
            4 => four = sort_segment_input(line),
            5 => five_digit_strings.push(sort_segment_input(line)),
            6 => six_digit_strings.push(sort_segment_input(line)),
            7 => eight = sort_segment_input(line),
            _ => {}
        }
    }

    for segment in six_digit_strings {
        if segment_contains(&segment, &four) {
            nine = segment;
        } else if segment_contains(&segment, &seven) {
            zero = segment;
        } else {
            six = segment;
        }
    }

    for segment in five_digit_strings {
        if segment_contains(&segment, &seven) {
            three = segment;
        } else if segment_contains(&six, &segment) {
            five = segment;
        } else {
            two = segment;
        }
    }

    map.entry(&zero).or_insert(0);
    map.entry(&one).or_insert(1);
    map.entry(&two).or_insert(2);
    map.entry(&three).or_insert(3);
    map.entry(&four).or_insert(4);
    map.entry(&five).or_insert(5);
    map.entry(&six).or_insert(6);
    map.entry(&seven).or_insert(7);
    map.entry(&eight).or_insert(8);
    map.entry(&nine).or_insert(9);

    let outputs = tokens.next().unwrap();
    let outputs:Vec<&str> = outputs.split_whitespace().collect();

    let mut result = map.get(sort_segment_input(outputs[0]).as_str()).unwrap() * 1000;
    result += map.get(sort_segment_input(outputs[1]).as_str()).unwrap() * 100;
    result += map.get(sort_segment_input(outputs[2]).as_str()).unwrap() * 10;
    result += map.get(sort_segment_input(outputs[3]).as_str()).unwrap();

    result
}

fn segment_contains(segment_a: &str, segment_b: &str) -> bool {
    for letter in segment_b.chars() {
        if !segment_a.contains(letter) {
            return false;
        }
    }
    true
}

fn sort_segment_input(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));

    String::from_iter(chars)
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 8 part one: {}", result);

    let result = part_two(&buffer);
    println!("Day 8 part two: {}", result);
}

#[test]
fn test_part_one() {
    let input = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
    assert_eq!(26, part_one(&input));
}

#[test]
fn test_sort_segment_input() {
    assert_eq!("abcd", sort_segment_input("cbda"));
    assert_eq!("abcd", sort_segment_input("abcd"));
}

#[test]
fn test_segment_contains() {
    assert_eq!(true, segment_contains("bcdef", "bde"));
    assert_eq!(false, segment_contains("abcdf", "bde"));
}

#[test]
fn test_compute_display_value() {
    let input = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
    assert_eq!(8394, compute_display_values(&input));
}

#[test]
fn test_part_two() {
    let input = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
    assert_eq!(61229, part_two(&input));
}