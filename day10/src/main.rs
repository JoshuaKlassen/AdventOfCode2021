use std::io::{self, Read};

fn part_one(input: &str) -> usize {
    let mut illegal_paren_count = 0;
    let mut illegal_bracket_count = 0;
    let mut illegal_brace_count = 0;
    let mut illegal_arrow_count = 0;

    for line in input.lines() {
        let mut stack = Vec::<char>::new();

        for char in line.trim().chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),
                ')' => {
                    if stack.len() == 0 {
                        break;
                    }
                    if stack.pop().unwrap() != '(' {
                        illegal_paren_count += 1;
                        break;
                    }
                },
                ']' => {
                    if stack.len() == 0 {
                        break;
                    }
                    if stack.pop().unwrap() != '[' {
                        illegal_bracket_count += 1;
                        break;
                    }
                },
                '}' => {
                    if stack.len() == 0 {
                        break;
                    }
                    if stack.pop().unwrap() != '{' {
                        illegal_brace_count += 1;
                        break;
                    }
                },
                '>' => {
                    if stack.len() == 0 {
                        break;
                    }
                    if stack.pop().unwrap() != '<' {
                        illegal_arrow_count += 1;
                        break;
                    }
                },
                _ => panic!("Cannot process char: {}", char)
            }
        }
    }

    (illegal_paren_count * 3) + (illegal_bracket_count * 57) + (illegal_brace_count * 1197) + (illegal_arrow_count * 25137)
}

fn part_two(input: &str) -> usize {
    let mut scores = Vec::<usize>::new();
    for line in input.lines(){
        if let Some(score) = compute_closing_line_score(&line) {
            scores.push(score);
        }
    }

    scores.sort();
    let middle_score = *scores.get(scores.len() / 2).unwrap();

    middle_score
}

fn compute_closing_line_score(input: &str) -> Option<usize> {
    let mut stack = Vec::<char>::new();
    for char in input.trim().chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' => {
                if stack.len() == 0 {
                    return None
                }
                if stack.pop().unwrap() != '(' {
                    return None
                }
            },
            ']' => {
                if stack.len() == 0 {
                    return None
                }
                if stack.pop().unwrap() != '[' {
                    return None
                }
            },
            '}' => {
                if stack.len() == 0 {
                    return None
                }
                if stack.pop().unwrap() != '{' {
                    return None
                }
            },
            '>' => {
                if stack.len() == 0 {
                    return None
                }
                if stack.pop().unwrap() != '<' {
                    return None
                }
            },
            _ => panic!("Cannot process char: {}", char)
        }
    }

    if stack.len() == 0 {
        return None
    }

    let mut result = 0;
    while !stack.is_empty() {
        result *= 5;
        let char = stack.pop().unwrap();
        match char {
            '(' => result += 1,
            '[' => result += 2,
            '{' => result += 3,
            '<' => result += 4,
            _ => panic!("Cannot process char: {}", char)
        }
    }

    Some(result)
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading input");

    let result = part_one(&buffer);
    println!("Day 10 part one: {}", result);

    let result = part_two(&buffer);
    println!("Day 10 part two: {}", result);
}

#[test]
fn test_part_one() {
    let input = String::from("[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]");
        assert_eq!(26397, part_one(&input));
}

#[test]
fn test_compute_closing_line_score() {
    assert_eq!(Some(288957), compute_closing_line_score("[({(<(())[]>[[{[]{<()<>>"));
    assert_eq!(Some(5566), compute_closing_line_score("[(()[<>])]({[<{<<[]>>("));
    assert_eq!(Some(1480781), compute_closing_line_score("(((({<>}<{<{<>}{[]{[]{}"));
    assert_eq!(Some(995444), compute_closing_line_score("{<[[]]>}<{[{[{[]{()[[[]"));
    assert_eq!(Some(294), compute_closing_line_score("<{([{{}}[<[[[<>{}]]]>[]]"));
    assert_eq!(None, compute_closing_line_score("{([(<{}[<>[]}>{[]{[(<()>"));

}

#[test]
fn test_part_two() {
    let input = String::from("[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]");
        assert_eq!(288957, part_two(&input));
}