use std::io::{self, Read};
use std::collections::HashMap;

fn simulate_day(input: &mut Vec<i32>) { 
    let mut new_fish = Vec::<i32>::new();

    for fish_timer in input.iter_mut() {
        *fish_timer -= 1;

        if *fish_timer == -1 {
            *fish_timer = 6;
            new_fish.push(8);
        }
    }

    input.append(&mut new_fish);
}

fn simulate_days(input: &mut Vec<i32>, days: usize) {
    for _i in 0..days {
        simulate_day(input);
    }
}

fn optimize_data(input: Vec<usize>) -> HashMap<usize, usize> {
    let mut map = HashMap::<usize, usize>::new();

    for value in input {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    for i in 0..9 {
        map.entry(i).or_insert(0);
    }

    map
}

fn simulate_day_optimize(input: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut result = HashMap::<usize, usize>::new();
    
    // print_fish(&input);

    let new_fish = input.get(&0).unwrap();
    
    for i in (1..9).rev() {
        let value = input.get(&i).unwrap();
        result.entry(i-1).or_insert(*value);
    }

    result.entry(6).and_modify(|e| *e += new_fish);

    let count = result.entry(8).or_insert(0);
    *count += new_fish;

    result
}

fn simulate_days_optimize(input: HashMap<usize, usize>, days: usize) -> HashMap<usize, usize> {
    let mut result = input.clone();

    for _i in 0..days {
        // println!("Day: {}", i);
        result = simulate_day_optimize(result);
    }

    result
}

fn count_fish(input: HashMap<usize, usize>) -> usize {
    let mut count = 0;

    for value in input.values() {
        count += value;
    }

    count
}

fn part_one(input: &str) -> usize {
   let mut fish = input.trim().split(',').map(|s| s.parse::<i32>().unwrap()).collect();

    simulate_days(&mut fish, 80);

    fish.len()
}

fn part_two(input: &str) -> usize {
    let fish = input.trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let fish = optimize_data(fish);
    let fish = simulate_days_optimize(fish, 256);

    count_fish(fish)
}

#[warn(dead_code)]
fn print_fish(fish: &HashMap<usize, usize>) {
    for i in 0..9 {
        print!("{}:{},", i, fish.get(&i).unwrap());
    }
    println!("");
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let day_one_result = part_one(&buffer);
    println!("Day 6 part one: {}", day_one_result);

    let day_two_result = part_two(&buffer);
    println!("Day 6 part two: {}", day_two_result);
}

#[test]
fn test_simulate_day() {
    let mut fish = vec![3,4,3,1,2];
    simulate_day(&mut fish);
    assert_eq!(vec![2,3,2,0,1], fish);
    simulate_day(&mut fish);
    assert_eq!(vec![1,2,1,6,0,8], fish);
}

#[test]
fn test_simulate_days() {
    let mut fish = vec![3,4,3,1,2];
    simulate_days(&mut fish, 18);
    assert_eq!(26, fish.len());
}

#[test]
fn test_part_one() {
    let input = String::from("3,4,3,1,2");
    assert_eq!(5934, part_one(&input));
}

#[test]
fn test_optimize_data() {
    let fish = vec![3,4,3,1,2];
    let fish = optimize_data(fish);
    assert_eq!(1, *fish.get(&1).unwrap());
    assert_eq!(1, *fish.get(&2).unwrap());
    assert_eq!(2, *fish.get(&3).unwrap());
    assert_eq!(1, *fish.get(&4).unwrap());
}

#[test]
fn test_simulate_day_optimized() {
    let fish = vec![3,4,3,1,2];
    let fish = optimize_data(fish);
    let fish = simulate_day_optimize(fish);
    let fish = simulate_day_optimize(fish);
    let fish = simulate_day_optimize(fish);

    print_fish(&fish);
}

#[test]
fn test_simulate_days_optimized() {
    let fish = vec![3,4,3,1,2];
    let fish = optimize_data(fish);
    let fish = simulate_days_optimize(fish, 80);
    assert_eq!(5934, count_fish(fish));
}

#[test]
fn test_part_two() {
    let input = String::from("3,4,3,1,2");
    assert_eq!(26984457539, part_two(&input));
}