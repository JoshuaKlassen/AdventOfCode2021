use std::io::{self, Read};
use std::cmp;

fn compute_cost_to_move(position: i32, crabs: &Vec<i32>) -> i32 {
    let mut total_cost = 0;

    for crab in crabs {
        let cost = position - crab;
        let cost = cost.abs();
        total_cost += cost;
    }

    total_cost
}

fn compute_cost_to_move_with_increasing_rate(position: i32, crabs: &Vec<i32>) -> i32 {
    let mut total_cost = 0;

    for crab in crabs {
        let distance = position - crab;
        let distance = distance.abs();
        let cost: i32 = (1..distance+1).sum();
        total_cost += cost;
    }

    total_cost
}

fn part_one(input: &str) -> i32 {
    let mut min_cost = i32::MAX;

    let crabs = input.trim().split(',');
    let crabs = crabs.map(|s| s.parse::<i32>().unwrap());
    let crabs: Vec<i32> = crabs.collect();

    let mut min_index = 0;
    let mut max_index = 0;

    for crab in &crabs {
        if *crab < min_index {
            min_index = *crab;
        }

        if *crab > max_index {
            max_index = *crab;
        }
    }

    for i in min_index..max_index+1 {
        let cost = compute_cost_to_move(i, &crabs);  
        min_cost = cmp::min(cost, min_cost); 
    }

    min_cost
}

fn part_two(input: &str) -> i32 {
    let mut min_cost = i32::MAX;

    let crabs = input.trim().split(',');
    let crabs = crabs.map(|s| s.parse::<i32>().unwrap());
    let crabs: Vec<i32> = crabs.collect();

    let mut min_index = 0;
    let mut max_index = 0;

    for crab in &crabs {
        if *crab < min_index {
            min_index = *crab;
        }

        if *crab > max_index {
            max_index = *crab;
        }
    }

    for i in min_index..max_index+1 {
        let cost = compute_cost_to_move_with_increasing_rate(i, &crabs);  
        min_cost = cmp::min(cost, min_cost); 
    }

    min_cost
}



fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 7 part one: {}", result);
    let result = part_two(&buffer);
    println!("Day 7 part one: {}", result);
}


#[test]
fn test_compute_cost_to_move() {
    let crabs = vec![16,1,2,0,4,2,7,1,2,14];
    assert_eq!(37, compute_cost_to_move(2, &crabs));
}

#[test]
fn test_part_one() {
    let input = String::from("16,1,2,0,4,2,7,1,2,14");
    assert_eq!(37, part_one(&input));
}

#[test]
fn test_compute_cost_to_move_with_increasing_rate() {
    let crabs = vec![16,1,2,0,4,2,7,1,2,14];
    assert_eq!(206, compute_cost_to_move_with_increasing_rate(2, &crabs));
}

#[test]
fn test_part_two() {
    let input = String::from("16,1,2,0,4,2,7,1,2,14");
    assert_eq!(168, part_two(&input));
}