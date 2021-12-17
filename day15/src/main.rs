use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct RiskGraph {
    graph: Vec<Vec<usize>>,
}

impl RiskGraph {
    fn compute_smallest_risk_path(&self) -> usize {
        let mut unvisited = BinaryHeap::<RiskPoint>::new();
        let mut node_weights = HashMap::<Point, usize>::new();
        let mut path = HashMap::<Point, usize>::new();

        let start_point = Point{x:0, y: 0};
        path.entry(start_point).or_insert(0);
        let end_point = Point{x: self.graph[0].len()-1, y: self.graph.len()-1};

        //initialize
        for row in 0..self.graph.len() {
            for col in 0..self.graph[row].len() {
                let point = Point{x: col, y: row};
                node_weights.entry(point).or_insert(i32::MAX.try_into().unwrap());
            }
        }

        let start_risk = node_weights.entry(start_point).or_insert(0);
        *start_risk = 0;
        unvisited.push(RiskPoint{x: start_point.x, y: start_point.y, risk: 0});

        //dijijijkstras
        while !unvisited.is_empty() {
            let current_point = unvisited.pop().unwrap();
            let current_point: Point = Point {x: current_point.x, y: current_point.y};

            if current_point.x == end_point.x && current_point.y == end_point.y {
                let risk = node_weights.get(&current_point).unwrap();
                return *risk;
            }

            let neighbours = self.get_neighbour_points(&current_point);

            let current_risk = node_weights.get(&current_point).unwrap().to_owned();

            for neighbour in neighbours {
                let risk = node_weights.entry(neighbour).or_default();
                let edge_risk = self.get_risk_level_for_point(&neighbour).unwrap();

                let new_risk = current_risk + edge_risk;

                if new_risk < *risk {
                    *risk = new_risk;
                    path.entry(neighbour).or_insert(*risk);
                    unvisited.push(RiskPoint {x: neighbour.x, y: neighbour.y, risk: *risk});
                }
            }
        }
        0
    }

    fn get_neighbour_points(&self, point: &Point) -> Vec<Point> {
        let mut points = Vec::<Point>::new();

        //up
        if point.y > 0 {
            points.push(Point{x: point.x, y: point.y -1});
        }

        //down
        if point.y < self.graph.len()-1 {
            points.push(Point{x: point.x, y: point.y +1});
        }

        //left
        if point.x > 0 {
            points.push(Point{x: point.x -1, y: point.y});
        }

        //down
        if point.x < self.graph[point.y].len()-1 {
            points.push(Point{x: point.x +1, y: point.y});
        }

        points
    }

    fn get_risk_level_for_point(&self, point: &Point) -> Option<usize> {
        if point.y > self.graph.len() {
            return None;
        }

        if point.x > self.graph.first().unwrap().len() {
            return None;
        }

        Some(self.graph[point.y][point.x])
    }

    fn expand(&mut self) {
        let mut new_graph = Vec::<Vec<usize>>::new();
        let width = self.graph[0].len();
        let height = self.graph.len();
        let mut j = 0;
        for row in 0..self.graph.len() * 5 {
            let mut new_row = Vec::<usize>::new();
            for col in 0..self.graph[0].len() * 5 {
                let i = col / width;
                let j = row / height;
                let original_col = col % width;
                let original_row = row % height;


                let original_value = self.graph[original_row][original_col];

                let mut value = (original_value + i + j);
                if value > 9 {
                    value = value % 9;
                }
                new_row.push(value);
            }
            new_graph.push(new_row);
        }

        self.graph = new_graph;
    }
}

impl FromStr for RiskGraph {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Vec::<Vec<usize>>::new();

        for line in s.lines() {
            let line = line.trim();
            let mut row = Vec::<usize>::new();
            for char in line.chars() {
                let value: usize = char.to_digit(10).unwrap()
                    .try_into()
                    .expect("Error reading value");
                    row.push(value);
            }
            graph.push(row);
        }

        Ok(RiskGraph{graph})
    }
}

fn part_one(input: &str) -> usize {
    let risk_graph: RiskGraph = input.trim().parse().expect("Error parsing Risk Graph");

    risk_graph.compute_smallest_risk_path()
}

fn part_two(input: &str) -> usize {
    let mut risk_graph: RiskGraph = input.trim().parse().expect("Error parsing Risk Graph");
    risk_graph.expand();

    // for row in &risk_graph.graph {
    //     for col in row {
    //         print!("{}", col);
    //     }
    //     println!();
    // }

    risk_graph.compute_smallest_risk_path()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 15 part one: {}", result);

    let result = part_two(&buffer);
    println!("Day 15 part two: {}", result);
}

#[test]
fn test_part_one() {
    let input = String::from("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581");
    assert_eq!(40, part_one(&input));
}

#[test]
fn test_part_two() {
    let input = String::from("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581");
    assert_eq!(315, part_two(&input));
}

struct RiskPoint {
    x: usize,
    y: usize,
    risk: usize,
}

impl PartialEq for RiskPoint {
    fn eq(&self, other: &Self) -> bool {
        self.risk == other.risk
    }
}

impl Eq for RiskPoint {}

impl PartialOrd for RiskPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.risk.cmp(&other.risk).reverse())
    }
}

impl Ord for RiskPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk.cmp(&other.risk).reverse()
    }
}

#[test]
fn test_bin_heap() {
    let mut bin = BinaryHeap::<RiskPoint>::new();

    let rp1 = RiskPoint {x: 0, y: 0, risk: 100};
    let rp2 = RiskPoint {x: 1, y: 5, risk: 50};
    let rp3 = RiskPoint {x: 5, y: 1, risk: 500};
    bin.push(rp1);
    assert_eq!(100, bin.peek().unwrap().risk);
    bin.push(rp2);
    bin.push(rp3);
    assert_eq!(50, bin.peek().unwrap().risk)
}