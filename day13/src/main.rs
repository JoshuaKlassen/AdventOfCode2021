use std::{io::{self, Read}, str::FromStr};
use std::cmp;

enum Axis {
    X,
    Y,
}

struct Command {
    axis: Axis,
    line: usize,
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut tokens = s.split_whitespace();
        tokens.next();
        tokens.next();

        let command = tokens.next().expect("Error reading command");
        let mut command = command.split("=");
        let axis_str = command.next().expect("Error reading axis");
        let mut axis: Axis = Axis::X;
        match axis_str {
            "x" => (),
            "y" => axis = Axis::Y,
            _ => return Err(String::from("Could not parse axis"))
        }

        let line = command.next().expect("Error reading coordinate");
        let line: usize = line.parse().expect("Error parsing coordinate");

        Ok(Command {axis, line})
    }
}

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

struct Manual {
    paper: Vec<Vec<bool>>,
}

impl FromStr for Manual {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Vec::<Vec<bool>>::new();
        let mut points = Vec::<Point>::new();
    
        let mut largest_x = 0;
        let mut largest_y = 0;
    
        for line in s.lines() {
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
    
        Ok(Manual {paper: result})   
    }
}

impl Manual {
    fn fold(&mut self, command: &Command) {
        match command.axis {
            Axis::X => self.fold_paper_vertical(command.line),
            Axis::Y => self.fold_paper_horizontal(command.line),
        }
    }

    fn fold_paper_horizontal(&mut self, y: usize) {
        if y > self.paper.len() {
            panic!("Cannot fold paper beyond bounds! Paper size: {}x{}, requested fold along y={}", self.paper.len(), self.paper.first().unwrap().len(), y);
        }
    
        let mut row_index = 0;
        for i in (y+1..self.paper.len()).rev() {
            for j in 0..self.paper[i].len() {
                let value = self.paper[i][j];
                if value {
                    self.paper[row_index][j] = true;
                }
            }
            row_index += 1;
            self.paper.pop();
        }
        self.paper.pop();
    }

    fn fold_paper_vertical(&mut self, x: usize) {
        if x > self.paper.first().unwrap().len() {
            panic!("Cannot fold paper beyond bounds! Paper size: {}x{}, requested fold along x={}", self.paper.len(), self.paper.first().unwrap().len(), x);
        }
    
        for i in 0..self.paper.len() {
            let mut col_index = 0;
            for _j in (x+1..self.paper[i].len()).rev() {
                let value = self.paper[i].pop().expect("Error reading paper");
                if value {
                    self.paper[i][col_index] = true;
                }
                col_index += 1;
            }
            self.paper[i].pop();
        }
    }

    fn print_paper(&self) {
        for row in &self.paper {
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
    
    fn count_points_on_paper(&self) -> i32 {
        let mut count = 0;
    
        for row in &self.paper {
            for col in row {
                if *col {
                    count += 1;
                }
            }
        }
    
        count
    }
}

fn part_one(input: &str) -> i32 {
    let mut tokens = input.split("\n\n");
    let manual = tokens.next().expect("Failed reading paper");
    let mut manual: Manual = manual.parse().expect("Error parsing manual");
    
    let commands = tokens.next().expect("Failed to parse commands");
    let mut commands = commands.split("\n");

    let command = commands.next().expect("Error reading command");
    let command: Command = command.parse().expect("Error parsing command");
    manual.fold(&command);
    
    manual.count_points_on_paper()
}

fn part_two(input: &str) {
    let mut tokens = input.split("\n\n");
    let manual = tokens.next().expect("Failed reading paper");
    let mut manual: Manual = manual.parse().expect("Error parsing manual");
    
    let commands = tokens.next().expect("Failed to parse commands");
    let commands = commands.split("\n");

    for command in commands {
        let command: Command = command.parse().expect("Error parsing command");
        manual.fold(&command);
    }

    manual.print_paper();
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

    let manual: Manual = input.parse().unwrap(); 
    let paper = &manual.paper;

    assert_eq!(15, paper.len());
    assert_eq!(11, paper.first().unwrap().len());
    assert_eq!(true, paper[0][3]);
    assert_eq!(18, manual.count_points_on_paper());
    manual.print_paper()
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

    let mut manual: Manual = input.parse().unwrap(); 
    manual.fold_paper_horizontal(7);
    assert_eq!(17, manual.count_points_on_paper());

    manual.fold_paper_vertical(5);
    manual.print_paper();
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
