use std::io::{self, Read};

fn part_one(input: &str) -> u32 {
    let mut low_points = Vec::<u32>::new();
    let mut rows = Vec::<Vec<u32>>::new();

    for line in input.lines() {
        let row:Vec<u32> = line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
        rows.push(row);
    }
    
    for (row_index, row) in rows.iter().enumerate() {
        let mut prev_row = Vec::<u32>::new();
        let mut next_row = Vec::<u32>::new();
        for _i in 0..row.len() {
            prev_row.push(9);
            next_row.push(9);
        }

        if row_index > 0 {
            prev_row = rows[row_index-1].clone();
        }

        if row_index < rows.len() - 1{
            next_row = rows[row_index+1].clone();
        }

        for (index, point) in row.iter().enumerate() {
            let mut left = 9;
            let up = prev_row[index];
            let mut right = 9;
            let down = next_row[index];
            if index == 0 {
                right = row[index+1];
            } else if index == row.len() - 1 {
                left = row[index-1];
            } else {
                left = row[index-1];
                right = row[index+1];

            }
            if point < &left && point < &right && point < &up && point < &down {
                low_points.push(*point+1);
            }
        }
    }

    low_points.iter().sum()
}

fn part_two(input: &str) -> u32 {
    let mut rows = Vec::<Vec<u32>>::new();

    let mut basins = Vec::<u32>::new();

    for line in input.lines() {
        if rows.len() == 0 {
            let mut row = Vec::<u32>::new();
            for _i in 0..line.trim().len() + 1 {
                row.push(9);
            }
            rows.push(row);
        }

        let mut row = Vec::<u32>::new();
        row.push(9);

        for char in line.trim().chars() {
            row.push(char.to_digit(10).unwrap());
        }
        row.push(9);
        rows.push(row);
    }

    let mut last_row = Vec::<u32>::new();
    for _i in 0..rows.first().unwrap().len() {
        last_row.push(9);
    }
    rows.push(last_row);
    
    //99999
    //9...9
    //9...9
    //99999

    for (row_index, row) in rows.iter().enumerate() {
        if row_index == 0 || row_index == rows.len() -1 {
            continue;
        }

        let prev_row = &rows[row_index-1];
        let next_row = &rows[row_index+1];

        for (col_index, point) in row.iter().enumerate() {
            if col_index == 0 || col_index == row.len() - 1 {
                continue;
            }
            let left = row[col_index-1];
            let up = prev_row[col_index];
            let right = row[col_index+1];
            let down = next_row[col_index];

            
            if point < &left && point < &right && point < &up && point < &down {
                let basin_size = compute_basin(&rows, row_index, col_index);
                basins.push(basin_size);
            }
        }
    }

    basins.sort_by(|a, b| b.cmp(a));

    basins[0] * basins[1] * basins[2]
}

fn compute_basin(rows: &Vec<Vec<u32>>, row_index: usize, col_index: usize) -> u32 {
    let mut basin_size = 0;
    let mut points_checked = Vec::<(usize, usize)>::new();

    compute_basin_recusive(&rows, row_index, col_index, &mut points_checked, &mut basin_size);

    basin_size
}

fn compute_basin_recusive(rows: &Vec<Vec<u32>>, row_index: usize, col_index: usize, points_checked: &mut Vec<(usize, usize)>, basin_size: &mut u32) {
    let current_point = rows[row_index][col_index];

    if current_point == 9 {
        return;
    }

    if points_checked.contains(&(row_index, col_index)) {
        return;
    }

    *basin_size += 1;
    points_checked.push((row_index, col_index));
    
    let left_point = rows[row_index][col_index-1];
    let up_point = rows[row_index-1][col_index];
    let right_point = rows[row_index][col_index+1];
    let down_point = rows[row_index+1][col_index];

    if left_point > current_point && left_point != 9 {
        compute_basin_recusive(&rows, row_index, col_index-1, points_checked, basin_size);
    }

    if right_point > current_point && right_point != 9{
        compute_basin_recusive(&rows, row_index, col_index+1, points_checked, basin_size);
    }

    if up_point > current_point && up_point != 9 {
        compute_basin_recusive(&rows, row_index-1, col_index, points_checked, basin_size);
    }

    if down_point > current_point && down_point != 9 {
        compute_basin_recusive(&rows, row_index+1, col_index, points_checked, basin_size);
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 9 part one: {}", result);

    let result = part_two(&buffer);
    println!("Day 9 part two: {}", result);
}

#[test]
fn test_part_one() {
    let input = String::from("2199943210
    3987894921
    9856789892
    8767896789
    9899965678");
    assert_eq!(15, part_one(&input));
}

#[test]
fn test_compute_basin_size() {
    let mut rows = Vec::<Vec<u32>>::new();
    rows.push(vec![9,9,9,9,9,9,9,9,9,9,9,9]);
    rows.push(vec![9,2,1,9,9,9,4,3,2,1,0,9]);
    rows.push(vec![9,3,9,8,7,8,9,4,9,2,1,9]);
    rows.push(vec![9,9,8,5,6,7,8,9,8,9,2,9]);
    rows.push(vec![9,8,7,6,7,8,9,6,7,8,9,9]);
    rows.push(vec![9,9,8,9,9,9,6,5,6,7,8,9]);
    rows.push(vec![9,9,9,9,9,9,9,9,9,9,9,9]);
    
    
    assert_eq!(9, compute_basin(&rows, 1, 10));
    assert_eq!(3,  compute_basin(&rows, 1, 2));
    assert_eq!(14,  compute_basin(&rows, 3, 3));
    assert_eq!(9,  compute_basin(&rows, 5, 7));
}

#[test]
fn test_part_two() {
    let input = String::from("2199943210
    3987894921
    9856789892
    8767896789
    9899965678");
    assert_eq!(1134, part_two(&input));
}
