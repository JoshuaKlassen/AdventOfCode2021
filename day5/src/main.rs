use std::{io::{self, Read}, str::FromStr, collections::HashMap};
use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tokens = input.split(',');
        let x: i32 = tokens.next().unwrap().trim().parse().expect("Failed to parse x coordinate");
        let y: i32 = tokens.next().unwrap().trim().parse().expect("Failed to parse y coordinate");

        Ok (Point { x, y })
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tokens = input.split("->");
        let start: Point = tokens.next().unwrap().parse().expect("Failed to parse starting point");
        let end: Point = tokens.next().unwrap().parse().expect("Failed to parse ending point");
        
        Ok (Line { start, end })
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn get_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();

        let horizontal_diff = self.start.x - self.end.x;
        let horizontal_diff = horizontal_diff.abs();

        let vertical_diff = self.start.y - self.end.y;
        let vertical_diff = vertical_diff.abs();

        if horizontal_diff == 0 && vertical_diff != 0 {
            let min = cmp::min(self.start.y, self.end.y);
            for index in 0..vertical_diff+1 {
                let new_point = Point { x: self.start.x, y: min + index };
                points.push(new_point);
            }

        } else if horizontal_diff != 0 && vertical_diff == 0 {
            let min = cmp::min(self.start.x, self.end.x);
            for index in 0..horizontal_diff+1 {
                let new_point = Point { x: min + index, y: self.start.y };
                points.push(new_point);
            }
        } else if horizontal_diff != 0 && vertical_diff != 0 {
            for index in 0..horizontal_diff+1 {
                let mut adjust_x = index;
                let mut adjust_y = index;

                if self.start.x > self.end.x {
                    adjust_x = -adjust_x;
                }

                if self.start.y > self.end.y {
                    adjust_y = -adjust_y;
                }

                let new_point = Point { x: self.start.x + adjust_x, y: self.start.y + adjust_y };
                points.push(new_point);
            }
        }
        points
    }
}

fn part_one(input: &str) -> usize {
    let mut points = HashMap::<Point, i32>::new();
    for input_line in input.lines() {
        let line: Line = input_line.trim().parse().expect("Failed to parse input line");

        if line.is_horizontal() || line.is_vertical() {
            let line_points = line.get_points();
            for point in line_points {
                let value = points.entry(point).or_insert(0);
                *value += 1;
            }
        }
    }

    points.values().filter(|count| **count > 1).count()
}

fn part_two(input: &str) -> usize {
    let mut points = HashMap::<Point, i32>::new();
    for input_line in input.lines() {
        let line: Line = input_line.trim().parse().expect("Failed to parse input line");
        let line_points = line.get_points();
        for point in line_points {
            let value = points.entry(point).or_insert(0);
            *value += 1;
        }
    }

    points.values().filter(|count| **count > 1).count()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let part_one_result = part_one(&buffer);
    println!("Day 5 part one: {}", part_one_result);

    let part_two_result = part_two(&buffer);
    println!("Day 5 part two: {}", part_two_result);
}

#[test]
fn test_get_points() {
    let line_string = String::from("0,9 -> 5,9");
    let line: Line = line_string.parse().expect("Test failed to parse line");
    assert_eq!(true, line.is_horizontal());
    assert_eq!(false, line.is_vertical());

    let points = line.get_points();
    assert_eq!(6, points.len());

    let line_string = String::from("7,0 -> 7,4");
    let line: Line = line_string.parse().expect("Test failed to parse line");
    assert_eq!(true, line.is_vertical());
    assert_eq!(false, line.is_horizontal());

    let points = line.get_points();
    assert_eq!(5, points.len());

    let line_string = String::from("9,7 -> 7,9");
    let line: Line = line_string.parse().expect("Test failed to parse line");
    assert_eq!(false, line.is_vertical());
    assert_eq!(false, line.is_horizontal());

    let points = line.get_points();
    assert_eq!(3, points.len());
}

#[test]
fn test_part_one() {
    let input = String::from("0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2");
    let result = part_one(&input);
    assert_eq!(5, result);
}

#[test]
fn test_part_two() {
    let input = String::from("0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2");
    let result = part_two(&input);
    assert_eq!(12, result);
}