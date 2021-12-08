use std::{io::{self, Read}, str::FromStr};

struct Bingo {
    board: Vec<(i32, bool)>,
}

impl FromStr for Bingo {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.lines().count() != 5 {
            return Err(format!("Cannot create board without 5 rows: {}", input));
        }

        let mut board = Vec::<(i32, bool)>::new();

        for line in input.lines() {
            let tokens: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();

            if tokens.len() != 5 {
                return Err(format!("Cannot create board without 5 columns: {}", input));
            }

            board.push((tokens[0], false));
            board.push((tokens[1], false));
            board.push((tokens[2], false));
            board.push((tokens[3], false));
            board.push((tokens[4], false));
        }

        Ok(Bingo { board })
    }
}

impl Bingo {
    fn create_from_string(input: &str) -> Bingo {
        if input.lines().count() != 5 {
            panic!("Cannot create board without 5 rows: {}", input);
        }

        let mut board = Vec::<(i32, bool)>::new();

        for line in input.lines() {
            let tokens: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();

            if tokens.len() != 5 {
                panic!("Cannot create board without 5 columns: {}", input);
            }

            board.push((tokens[0], false));
            board.push((tokens[1], false));
            board.push((tokens[2], false));
            board.push((tokens[3], false));
            board.push((tokens[4], false));
        }

        Bingo { board }
    }

    fn get_row(&self, row_index: usize) -> Vec<(i32, bool)> {
        if row_index > 4 {
            panic!("Canot get nth row greater than 4: {}", row_index);
        }
        
        let mut row = Vec::<(i32, bool)>::new();

        let row_index = row_index * 5;

        row.push(self.board[row_index]);
        row.push(self.board[row_index + 1]);
        row.push(self.board[row_index + 2]);
        row.push(self.board[row_index + 3]);
        row.push(self.board[row_index + 4]);

        row
    }

    fn get_col(&self, col_index: usize) -> Vec<(i32, bool)> {
        if col_index > 4 {
            panic!("Cannot get nth col greater than 4: {}", col_index);
        }
        
        let mut col = Vec::<(i32, bool)>::new();

        col.push(self.board[col_index]);
        col.push(self.board[col_index + 5]);
        col.push(self.board[col_index + 10]);
        col.push(self.board[col_index + 15]);
        col.push(self.board[col_index + 20]);

        col
    }

    fn mark_number_as_called(&mut self, number_called: i32) {
        for value in &mut self.board {
            if value.0 == number_called {
                *value = (value.0, true);
                return
            }
        }
    }

    fn is_winning_straight(values: Vec<(i32, bool)>) -> bool {
        for value in values {
            if value.1 == false {
                return false
            }
        }

        true
    }

    fn is_winning_game(&self) -> bool {   
        let mut index = 0;
        while index < 5 {
            let row = self.get_row(index);
            if Bingo::is_winning_straight(row) {
                return true
            }

            let col = self.get_col(index);
            if Bingo::is_winning_straight(col) {
                return true
            }

            index += 1;
        }

        false
    }

    fn compute_uncalled_sum(&self) -> i32 {
        let mut sum = 0;

        for value in &self.board {
            if value.1 == false {
                sum += value.0;
            }
        }

        sum
    }
}

fn part_one(input: &str) -> i32 {
    let mut boards = Vec::<Bingo>::new();

    let mut iter = input.split("\n\n");
    let first_line = iter.next().unwrap();
    let call_numbers: Vec<i32> = first_line
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for board_string in iter {
        let board: Bingo = board_string.parse().unwrap();
        boards.push(board);
    }

    for number in call_numbers {
        for board in &mut boards {
            board.mark_number_as_called(number);

            if board.is_winning_game() {
                return board.compute_uncalled_sum() * number
            }
        }
    }

    0
}

fn part_two(input: &str) -> i32 {
    let mut boards = Vec::<Bingo>::new();

    let mut iter = input.split("\n\n");
    let first_line = iter.next().unwrap();
    let call_numbers: Vec<i32> = first_line
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for board_string in iter {
        let board = Bingo::create_from_string(&board_string);
        boards.push(board);
    }

    for number in call_numbers {
        for board in &mut boards {
            board.mark_number_as_called(number);
        }

        if boards.len() != 1 {
            boards.retain(|board| {
                !board.is_winning_game()
            });
        } else {
            let last_board = boards.first().unwrap();
            if last_board.is_winning_game() {
                return last_board.compute_uncalled_sum() * number
            }
        }
    }

    0
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Could not read input");

    let part_one_result = part_one(&buffer);
    println!("Day 4 part one: {}", part_one_result);

    let part_two_result = part_two(&buffer);
    println!("Day 4 part two: {}", part_two_result);
}

#[test]
fn test_create_from_string() {
    let input = String::from(
    "22 13 17 11  0
    8  2 23  4 24
   21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19");

    let bingo = Bingo::create_from_string(&input);

    assert_eq!(25, bingo.board.len())
}

#[test]
fn test_get_row() {
    let input = String::from(
        "22 13 17 11  0
        8  2 23  4 24
       21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19");
    
    let bingo = Bingo::create_from_string(&input);
    
    let row_1 = bingo.get_row(0);
    assert_eq!(22, row_1[0].0);
    assert_eq!(13, row_1[1].0);
    assert_eq!(17, row_1[2].0);
    assert_eq!(11, row_1[3].0);
    assert_eq!(0, row_1[4].0);
}

#[test]
fn test_get_col() {
    let input = String::from(
        "22 13 17 11  0
        8  2 23  4 24
       21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19");
    
    let bingo = Bingo::create_from_string(&input);
    
    let col_3 = bingo.get_col(2);
    assert_eq!(17, col_3[0].0);
    assert_eq!(23, col_3[1].0);
    assert_eq!(14, col_3[2].0);
    assert_eq!(3, col_3[3].0);
    assert_eq!(20, col_3[4].0);
}

#[test]
fn mark_number_as_called() {
    let input = String::from(
        "22 13 17 11  0
        8  2 23  4 24
       21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19");
    
    let mut bingo = Bingo::create_from_string(&input);

    bingo.mark_number_as_called(2);
    assert_eq!((2, true), bingo.get_row(1)[1]);

    bingo.mark_number_as_called(5);
    assert_eq!((5, true), bingo.get_col(4)[3]);
}

#[test]
fn test_winning_straight() {
    let input = String::from(
        "22 13 17 11  0
        8  2 23  4 24
       21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19");
    
    let mut bingo = Bingo::create_from_string(&input);

    bingo.mark_number_as_called(11);
    bingo.mark_number_as_called(4);
    bingo.mark_number_as_called(16);
    bingo.mark_number_as_called(18);
    assert_eq!(false, Bingo::is_winning_straight(bingo.get_col(3)));

    bingo.mark_number_as_called(15);
    assert_eq!(true, Bingo::is_winning_straight(bingo.get_col(3)));

    assert_eq!(true, bingo.is_winning_game());
}

#[test]
fn test_compute_uncalled_sum() {
    let input = String::from(
        "14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7");
    
    let mut bingo = Bingo::create_from_string(&input);

    bingo.mark_number_as_called(7);
    bingo.mark_number_as_called(4);
    bingo.mark_number_as_called(9);
    bingo.mark_number_as_called(5);
    bingo.mark_number_as_called(11);
    bingo.mark_number_as_called(17);
    bingo.mark_number_as_called(23);
    bingo.mark_number_as_called(2);
    bingo.mark_number_as_called(0);
    bingo.mark_number_as_called(14);
    bingo.mark_number_as_called(21);
    bingo.mark_number_as_called(24);

    assert_eq!(true, bingo.is_winning_game());
    assert_eq!(188, bingo.compute_uncalled_sum());
}

#[test]
fn test_part_one() {
    let input = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");

    assert_eq!(4512, part_one(&input));
}

#[test]
fn test_part_two() {
    let input = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");

    assert_eq!(1924, part_two(&input));
}