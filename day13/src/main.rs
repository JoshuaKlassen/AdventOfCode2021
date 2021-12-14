use std::{io::{self, Read}, str::FromStr};
use std::cmp;

struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(",");
        let x = tokens.next().expect("Error reading x");
        let x: i32 = x.parse().expect("Error parsing x");
        let y = tokens.next().expect("Error reading y");
        let y: i32 = y.parse().expect("Error parsing y");

        Ok(Point {x, y})
    }
}

fn part_one(input: &str) -> i32 {
    let mut tokens = input.split("\n\n");
    let paper = tokens.next().expect("Failed reading paper");
    let mut paper = parse_paper(&paper);
    
    let commands = tokens.next().expect("Failed to parse commands");
    let mut commands = commands.split("\n");

    let command = commands.next().expect("Error reading command");
    let mut tokens = command.split_whitespace();
    tokens.next();
    tokens.next();
    let line = tokens.next().expect("Error reading command");
    let mut line = line.split("=");
    let axis = line.next().expect("Error reading axis");
    let axis: char = axis.parse().expect("Error parsing axis");
    let coord = line.next().expect("Error reading coordinate");
    let coord: usize = coord.parse().expect("Error parsing coordinate");

    match axis {
        'x' => fold_paper_vertical(&mut paper, coord),
        'y' => fold_paper_horizontal(&mut paper, coord),
        _ => {}
    }
    
    count_points_on_paper(&paper)
}

fn parse_paper(input: &str) -> Vec<Vec<bool>> {
    let mut result = Vec::<Vec<bool>>::new();
    let mut points = Vec::<Point>::new();

    let mut largest_x = 0;
    let mut largest_y = 0;

    for line in input.lines() {
        let point: Point = line.trim().parse().expect("Error reading point");
        largest_x = cmp::max(largest_x, point.x);
        largest_y = cmp::max(largest_y, point.y);
        points.push(point);
    }

    for _i in 0..largest_y + 1 {
        let mut row = Vec::<bool>::new();
        for _j in 0..largest_x + 1{
            row.push(false);
        }
        result.push(row);
    }

    for point in points {
        result[point.y as usize][point.x as usize] = true;
    }

    result
}

fn part_two(input: &str) {
    let mut tokens = input.split("\n\n");
    let paper = tokens.next().expect("Failed reading paper");
    let mut paper = parse_paper(&paper);
    
    let commands = tokens.next().expect("Failed to parse commands");
    let commands = commands.split("\n");

    for command in commands {
        let command = command.trim();
        let mut tokens = command.split_whitespace();
        tokens.next();
        tokens.next();
        let line = tokens.next().expect("Error reading command");
        let mut line = line.split("=");
        let axis = line.next().expect("Error reading axis");
        let axis: char = axis.parse().expect("Error parsing axis");
        let coord = line.next().expect("Error reading coordinate");
        let coord: usize = coord.parse().expect("Error parsing coordinate");
    
        match axis {
            'x' => fold_paper_vertical(&mut paper, coord),
            'y' => fold_paper_horizontal(&mut paper, coord),
            _ => {}
        }
    }

    print_paper(&paper);
}

fn fold_paper_horizontal(paper: &mut Vec<Vec<bool>>, y: usize) {
    if y > paper.len() {
        panic!("Cannot fold paper beyond bounds! Paper size: {}x{}, requested fold along y={}", paper.len(), paper.first().unwrap().len(), y);
    }

    let mut row_index = 0;
    for i in (y+1..paper.len()).rev() {
        for j in 0..paper[i].len() {
            let value = paper[i][j];
            if value {
                paper[row_index][j] = true;
            }
        }
        row_index += 1;
        paper.pop();
    }
    paper.pop();
}

fn fold_paper_vertical(paper: &mut Vec<Vec<bool>>, x: usize) {
    if x > paper.first().unwrap().len() {
        panic!("Cannot fold paper beyond bounds! Paper size: {}x{}, requested fold along x={}", paper.len(), paper.first().unwrap().len(), x);
    }

    for i in 0..paper.len() {
        let mut col_index = 0;
        for _j in (x+1..paper[i].len()).rev() {
            let value = paper[i].pop().expect("Error reading paper");
            if value {
                paper[i][col_index] = true;
            }
            col_index += 1;
        }
        paper[i].pop();
    }
}

fn print_paper(paper: &Vec<Vec<bool>>) {
    for row in paper {
        for col in row {
            if *col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn count_points_on_paper(paper: &Vec<Vec<bool>>) -> i32 {
    let mut count = 0;

    for row in paper {
        for col in row {
            if *col {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 13 part one: {}", result);

    println!("Day 13 part two: ");
    part_two(&buffer);
}

#[test]
fn test_parse_paper() {
    let input = String::from("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0");

    let paper = parse_paper(&input);

    assert_eq!(15, paper.len());
    assert_eq!(11, paper.first().unwrap().len());
    assert_eq!(true, paper[0][3]);
    assert_eq!(18, count_points_on_paper(&paper));
    print_paper(&paper);
}

#[test]
fn test_fold_paper() {
    let input = String::from("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0");

    let mut paper = parse_paper(&input);

    fold_paper_horizontal(&mut paper, 7);
    assert_eq!(17, count_points_on_paper(&paper));

    fold_paper_vertical(&mut paper, 5);
    print_paper(&paper);
}

#[test]
fn test_part_one() {
    let input = String::from("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5");

    assert_eq!(17, part_one(&input));
}
