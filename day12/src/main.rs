use std::{io::{self, Read}, str::FromStr, collections::HashMap, slice::SliceIndex};

struct Graph {
    nodes: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Graph {
        let mut graph = Graph { nodes: HashMap::<String, Vec<String>>::new() };

        graph.nodes.entry(String::from("start")).or_insert(Vec::<String>::new());
        graph.nodes.entry(String::from("end")).or_insert(Vec::<String>::new());

        graph
    }

    fn add_new_connection(&mut self, a: &str, b: &str) {
        let node_a = self.nodes.entry(String::from(a)).or_insert(Vec::<String>::new());
        node_a.push(String::from(b));

        let node_b = self.nodes.entry(String::from(b)).or_insert(Vec::<String>::new());
        node_b.push(String::from(a));
    }

    fn print_graph(&self) {
        for (key, value) in &self.nodes {
            println!("{}: {:?}", key, value);
        }
    }

    fn count_paths(&self) -> i32 {
        let mut path = Vec::<String>::new();
        self.count_paths_to_end(String::from("start"), &mut path)
    }

    fn count_paths_with_small_caves(&self) -> i32 {
        let mut visited = HashMap::<String, i32>::new();

        for (node, _ ) in &self.nodes {
            visited.entry(node.clone()).or_insert(0);
        }

        let mut path = Vec::<String>::new();

        self.count_paths_to_end_with_small_caves(String::from("start"), &mut visited, &mut path)
    }

    fn count_paths_to_end(&self, node: String, visited: &mut Vec<String>) -> i32 {
        let mut count = 0;
        if visited.contains(&node) && node.to_lowercase() == node {
            return 0;
        }

        if node == "end" {
            println!("{:?}", visited);
            return 1;
        }

        visited.push(node.clone());

        let children = self.nodes.get(&node).unwrap();
        for child in children {
            count += self.count_paths_to_end(child.clone(), visited);
        }

        visited.pop();

        count
    }

    fn count_paths_to_end_with_small_caves(&self, node: String, visited: &mut HashMap<String, i32>, path: &mut Vec<String>) -> i32 {
        let mut count = 0;

        let is_small_cave = node.to_lowercase() == node;

        if is_small_cave {
            let visits = visited.get(&node.clone()).unwrap();
            if *visits == 1 {
                for (key, value) in visited.iter() {
                    if *key.to_lowercase() == *key && *key != node {
                        if *value == 2 {
                            return 0; //been in a small cave twice already
                        }
                    }
                }
            }
        }

        if node == "end" {
            println!("{:?}", path);
            return 1;
        }

        let visits = visited.entry(node.clone()).or_insert(0);
        *visits += 1;

        path.push(node.clone());

        let children = self.nodes.get(&node).unwrap();
        for child in children {
            if child != "start" {
                if *child.to_lowercase() == *child {
                    if *visited.get(child).unwrap() < 2 {
                        count += self.count_paths_to_end_with_small_caves(child.clone(), visited, path);
                    }
                } else {
                    count += self.count_paths_to_end_with_small_caves(child.clone(), visited, path);
                }

            }
        }

        let visits = visited.entry(node).or_default();
        *visits -= 1;
        
        path.pop();

        count
    }

}

fn part_one(input: &str) -> i32 {
    let mut graph = Graph::new();

    for line in input.lines() {
        let mut tokens = line.split("-");
        let start_node_name = tokens.next().unwrap().trim();
        let end_node_name = tokens.next().unwrap().trim();

        graph.add_new_connection(start_node_name, end_node_name);

    }

    graph.count_paths()
}

fn part_two(input: &str) -> i32 {
    let mut graph = Graph::new();

    for line in input.lines() {
        let mut tokens = line.split("-");
        let start_node_name = tokens.next().unwrap().trim();
        let end_node_name = tokens.next().unwrap().trim();

        graph.add_new_connection(start_node_name, end_node_name);

    }

    graph.count_paths_with_small_caves()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 12 part one: {}", result);

    let result = part_two(&buffer);
    println!("Day 12 part two: {}", result);
}

#[test]
fn test_part_one() {
    let input = String::from("start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end");
    assert_eq!(10, part_one(&input));

    let input = String::from("dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc");
    assert_eq!(19, part_one(&input));

    let input = String::from("fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW");
    assert_eq!(226, part_one(&input));
}

#[test]
fn test_part_two() {
    let input = String::from("start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end");
    assert_eq!(36, part_two(&input));

    let input = String::from("dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc");
    assert_eq!(103, part_two(&input));

    let input = String::from("fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW");
    assert_eq!(3509, part_two(&input));
}