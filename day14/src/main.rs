use std::io::{self, Read};
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ElementPair {
    left: char,
    right: char,
}

impl ElementPair {
    fn to_string(&self) -> String {
        let mut result = String::from(self.left);
        result.push(self.right);

        result
    }
}

struct PolymerCommand {
    pair: ElementPair,
    insert: char,
}

impl FromStr for PolymerCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split_whitespace();
        let pair = tokens.next().unwrap();
        let mut chars = pair.chars();
        let pair = ElementPair {left: chars.next().unwrap(), right: chars.next().unwrap() };
        tokens.next();
        let insert = tokens.next().unwrap();
        let insert = insert.chars().next().unwrap();

        Ok(PolymerCommand{ pair, insert})
    }
}

struct Polymer {
    sequence: Vec<char>
}

impl FromStr for Polymer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.trim().chars();
        let chars: Vec<char> = chars.collect();
        Ok(Polymer {sequence: chars}) 
    }
}


impl Polymer {
    fn get_pairs(&self) -> Vec<ElementPair> {
        let mut result = Vec::<ElementPair>::new();

        for i in 1..self.sequence.len() {
            let left = self.sequence[i-1];
            let right = self.sequence[i];
            result.push(ElementPair{left, right});
        }

        result
    }

    fn get_quantities(&self) -> HashMap<char, i64> {
        let mut result = HashMap::<char, i64>::new();

        for &char in &self.sequence {
            let count = result.entry(char).or_insert(0);
            *count += 1;
        }

        result
    }

    fn compute_new_polymer(&self, commands: &Vec<PolymerCommand>) -> Self {
        let mut new_polymer = String::new();

        let pairs = self.get_pairs();
        let mut last_pair: Option<ElementPair> = None;

        for pair in pairs {
            let mut result = String::new();

            match last_pair {
                None => {
                    last_pair = Some(pair);
                    result.push(pair.left);
                },
                Some(_) => {}
            }

            for command in commands {
                if pair == command.pair {
                    result.push(command.insert);
                }
            }


            result.push(pair.right);
            new_polymer.push_str(&result);

        }

        let new_polymer: Polymer = new_polymer.parse().unwrap();

        new_polymer
    }

    fn to_string(&self) -> String {
        let mut result = String::new();

        for &char in &self.sequence {
            result.push(char);
        }

        result
    }
}

fn part_one(input: &str) -> i64 {
    let mut lines = input.lines();
    let polymer = lines.next().unwrap();
    let mut polymer: Polymer = polymer.parse().expect("Error parsing polymer");

    lines.next();

    let mut commands = Vec::<PolymerCommand>::new();
    for line in lines {
        let command: PolymerCommand = line.trim().parse().unwrap();
        commands.push(command);
    }

    for _i in 0..10 {
        polymer = polymer.compute_new_polymer(&commands);
    }

    let quantities = polymer.get_quantities();
    let mut highest_count = 0;
    let mut lowest_count = i64::MAX;

    for &count in quantities.values(){
        if count > highest_count {
            highest_count = count;
        } else if count < lowest_count {
            lowest_count = count;
        }
    }

    highest_count - lowest_count
}

struct CompressedUnorderedPolymer {
    occurences: HashMap<ElementPair, i64>,
    counts: HashMap<char, i64>,
    commands: Vec<PolymerCommand>,
}

impl CompressedUnorderedPolymer {
    fn from_polymer_and_commands(polymer: Polymer, commands: Vec<PolymerCommand>) -> Self {
        let mut occurences = HashMap::<ElementPair, i64>::new();
        let mut counts = HashMap::<char, i64>::new();

        for &char in &polymer.sequence {
            let count = counts.entry(char).or_insert(0);
            *count += 1;
        }


        let pairs = polymer.get_pairs();

        for pair in pairs {
            let count = occurences.entry(pair).or_insert(0);
            *count += 1;
        }

        CompressedUnorderedPolymer {
            occurences,
            counts,
            commands,
        }
    }

    fn simulate_update(&mut self) {
        let mut new_occurences = HashMap::<ElementPair, i64>::new();
        for occur in &self.occurences {
            new_occurences.entry(*occur.0).or_insert(*occur.1);
        }

        for &pair in self.occurences.keys() {
            let pair_count = self.occurences.get(&pair).unwrap();
            if *pair_count <= 0 {
                continue;
            }

            for command in &self.commands {
                if command.pair == pair {
                    let left_pair = ElementPair {left: pair.left, right: command.insert};
                    let right_pair = ElementPair {left: command.insert, right: pair.right};

                    let count = self.counts.entry(command.insert).or_insert(0);
                    *count += pair_count;

                    let count = new_occurences.entry(left_pair).or_insert(0);
                    *count += pair_count;

                    let count = new_occurences.entry(right_pair).or_insert(0);
                    *count += pair_count;

                    let count = new_occurences.entry(pair).or_default();
                    *count -= pair_count; //remove the old pair

                    break;
                }
            }
        }

        self.occurences = new_occurences;
    }

    fn get_quantities(&self) -> HashMap<char, i64> {
        let mut result = HashMap::<char, i64>::new();

        for (key, value) in self.counts.iter() {
            result.entry(*key).or_insert(*value);
        }

        result
    }
}

fn part_two(input: &str) -> i64 {
    let mut lines = input.lines();
    let polymer = lines.next().unwrap();
    let polymer: Polymer = polymer.parse().expect("Error parsing polymer");

    lines.next();

    let mut commands = Vec::<PolymerCommand>::new();
    for line in lines {
        let command: PolymerCommand = line.trim().parse().unwrap();
        commands.push(command);
    }

    let mut compressed_polymer = CompressedUnorderedPolymer::from_polymer_and_commands(polymer, commands);

    for _i in 0..40 {
        compressed_polymer.simulate_update();
    }
    let quantities = compressed_polymer.get_quantities();

    let mut highest_count = 0;
    let mut lowest_count = i64::MAX;

    for &count in quantities.values(){
        if count > highest_count {
            highest_count = count;
        } else if count < lowest_count {
            lowest_count = count;
        }
    }

    highest_count - lowest_count
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 14 part one: {}", result);

    let result = part_two(&buffer);
    println!("Day 14 part two: {}", result);
}

#[test]
fn test_get_pairs() {
    let polymer: Polymer = String::from("NNCB").parse().unwrap();
    let pairs = polymer.get_pairs();
    let first_pair = pairs[0].to_string();
    assert_eq!("NN", first_pair);
    let second_pair = pairs[1].to_string();
    assert_eq!("NC", second_pair);
    let third_pair = pairs[2].to_string();
    assert_eq!("CB", third_pair);
}

#[test]
fn test_compute_new_polymer() {
    let polymer: Polymer = String::from("NNCB").parse().unwrap();
    let command_1: PolymerCommand = String::from("NN -> C").parse().unwrap();
    let command_2: PolymerCommand = String::from("NC -> B").parse().unwrap();
    let command_3: PolymerCommand = String::from("CB -> H").parse().unwrap();
    let commands = vec![command_1, command_2, command_3];
    let polymer = polymer.compute_new_polymer(&commands);
    println!("{}", polymer.to_string());
}  

#[test]
fn test_part_one() {
    let input = String::from("NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C");
    assert_eq!(1588, part_one(&input));
}

#[test]
fn test_part_two() {
    let input = String::from("NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C");
    assert_eq!(2188189693529, part_two(&input));
}
