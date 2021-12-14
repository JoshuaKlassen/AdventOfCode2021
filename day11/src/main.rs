use std::io::{self, Read};

fn part_one(input: &str) -> i32 {
    let mut octs = Vec::<Vec<i32>>::new();
    let mut flashes = 0;

    for line in input.lines() {
        let oct_line:Vec<i32> = line.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        octs.push(oct_line);
    }

    for _i in 1..101 {
        flashes += simulate_day(&mut octs);
    }

    flashes
}

fn simulate_day(octs: &mut Vec<Vec<i32>>) -> i32 {
    let mut flashes = 0;

    for row in &mut *octs {
        for col in row {
            *col += 1;
        }
    }

    let mut flashed = Vec::<(usize, usize)>::new();
    for row_index in 0..octs.len() {
        for col_index in 0..octs.first().unwrap().len() {
            if octs[row_index][col_index] > 9 {
                flashes += flash_oct(octs, &mut flashed, row_index, col_index);
            }
        }
    }

    flashes
}

fn flash_oct(octs: &mut Vec<Vec<i32>>, seen: &mut Vec<(usize, usize)>, row_index: usize, col_index: usize) -> i32 {
    let mut flashes = 0;

    if row_index > octs.len() - 1{
        return 0;
    }

    if col_index > octs.first().unwrap().len() - 1{
        return 0;
    }

    if seen.contains(&(row_index, col_index)) {
        return 0;
    }

    octs[row_index][col_index] += 1;
    let current_oct = octs[row_index][col_index];

    if current_oct > 9 {
        flashes += 1;
        seen.push((row_index, col_index));
        if row_index == 0 && col_index == 0 {
            flashes += flash_oct(octs, seen, row_index+1, col_index);
            flashes += flash_oct(octs, seen, row_index, col_index+1);
            flashes += flash_oct(octs, seen, row_index+1, col_index+1);
        } else if row_index == 0 && col_index != 0 {
            flashes += flash_oct(octs, seen, row_index, col_index-1);
            flashes += flash_oct(octs, seen,row_index, col_index+1);
    
            flashes += flash_oct(octs, seen, row_index+1, col_index-1);
            flashes += flash_oct(octs, seen, row_index+1, col_index);
            flashes += flash_oct(octs, seen, row_index+1, col_index+1);
        } else if row_index != 0 && col_index == 0 {
            flashes += flash_oct(octs, seen, row_index-1, col_index);
            flashes += flash_oct(octs, seen, row_index-1, col_index+1);
    
            flashes += flash_oct(octs, seen, row_index, col_index+1);
    
            flashes += flash_oct(octs, seen, row_index+1, col_index);
            flashes += flash_oct(octs, seen, row_index+1, col_index+1);
        } else {
            flashes += flash_oct(octs, seen, row_index-1, col_index-1);
            flashes += flash_oct(octs, seen, row_index-1, col_index);
            flashes += flash_oct(octs, seen, row_index-1, col_index+1);
    
            flashes += flash_oct(octs, seen, row_index, col_index-1);
            flashes += flash_oct(octs, seen, row_index, col_index+1);
    
            flashes += flash_oct(octs, seen, row_index+1, col_index-1);
            flashes += flash_oct(octs, seen, row_index+1, col_index);
            flashes += flash_oct(octs, seen, row_index+1, col_index+1);
        }
        octs[row_index][col_index] = 0;
    }

    flashes
}

fn print_board(octs: &Vec<Vec<i32>>) {
    for row in octs {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn part_two(input: &str) -> u32 {
    let mut octs = Vec::<Vec<i32>>::new();

    for line in input.lines() {
        let oct_line:Vec<i32> = line.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        octs.push(oct_line);
    }
    let mut day = 0;
    while true {
        day += 1;
        let flashes = simulate_day(&mut octs);
        if flashes as usize == octs.len() * octs.first().unwrap().len() {
            break;
        }
    }

    day
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 11 part one: {}", result);

    
    let result = part_two(&buffer);
    println!("Day 11 part two: {}", result);
}

#[test]
fn test_simulate_day() {
    let mut octs = Vec::<Vec<i32>>::new();
    octs.push(vec![1,1,1,1,1]);
    octs.push(vec![1,9,9,9,1]);
    octs.push(vec![1,9,1,9,1]);
    octs.push(vec![1,9,9,9,1]);
    octs.push(vec![1,1,1,1,1]);

    let flashes = simulate_day(&mut octs);
    print_board(&octs);
    assert_eq!(9, flashes);

    let flashes = simulate_day(&mut octs);
    print_board(&octs);
    assert_eq!(0, flashes);
}

#[test]
fn test_part_one() {
    let input = String::from("5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526");
    assert_eq!(1656, part_one(&input));
}

#[test]
fn test_part_two() {
    let input = String::from("5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526");
    assert_eq!(195, part_two(&input));
}