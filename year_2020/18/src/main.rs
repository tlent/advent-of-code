static INPUT: &str = include_str!("../input");

fn main() {
    let sum: u64 = INPUT.lines().map(|l| solve(l)).sum();
    println!("part one: {}", sum);
    let sum: u64 = INPUT.lines().map(|l| solve_part_two(l)).sum();
    println!("part two: {}", sum);
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

fn solve(expression: &str) -> u64 {
    let mut stack = vec![];
    let mut left = None;
    let mut op = None;
    for c in expression.chars() {
        match c {
            ' ' => {}
            '(' => {
                stack.push((left, op));
                left = None;
                op = None;
            }
            ')' => {
                let right = left;
                let (l, o) = stack.pop().unwrap();
                left = l;
                op = o;
                if let Some(r) = right {
                    if let Some(l) = left {
                        left = Some(match op {
                            Some(Operation::Add) => l + r,
                            Some(Operation::Multiply) => l * r,
                            None => panic!("missing op"),
                        });
                    } else {
                        left = Some(r);
                    }
                }
            }
            '+' => op = Some(Operation::Add),
            '*' => op = Some(Operation::Multiply),
            '0'..='9' => {
                let v = c.to_digit(10).unwrap() as u64;
                if let Some(l) = left {
                    left = Some(match op {
                        Some(Operation::Add) => l + v,
                        Some(Operation::Multiply) => l * v,
                        None => panic!("missing op"),
                    });
                } else {
                    left = Some(v);
                }
            }
            c => panic!("invalid char: {}", c),
        };
    }
    left.unwrap()
}

#[derive(Debug)]
enum Token {
    Operator(Operation),
    Value(u64),
}

fn solve_part_two(expression: &str) -> u64 {
    let mut stack = vec![];
    let mut tokens = vec![];
    for c in expression.chars() {
        match c {
            ' ' => {}
            '(' => {
                stack.push(tokens);
                tokens = vec![];
            }
            ')' => {
                let mut mult_values = vec![];
                let mut current_sum = 0;
                for t in tokens {
                    match t {
                        Token::Operator(Operation::Add) => {}
                        Token::Value(v) => current_sum += v,
                        Token::Operator(Operation::Multiply) => {
                            mult_values.push(current_sum);
                            current_sum = 0;
                        }
                    };
                }
                mult_values.push(current_sum);
                let v = mult_values.iter().product();
                tokens = stack.pop().unwrap();
                tokens.push(Token::Value(v));
            }
            '+' => tokens.push(Token::Operator(Operation::Add)),
            '*' => tokens.push(Token::Operator(Operation::Multiply)),
            '0'..='9' => {
                let v = c.to_digit(10).unwrap() as u64;
                tokens.push(Token::Value(v));
            }
            c => panic!("invalid char: {}", c),
        };
    }
    let mut mult_values = vec![];
    let mut current_sum = 0;
    for t in tokens {
        match t {
            Token::Operator(Operation::Add) => {}
            Token::Value(v) => current_sum += v,
            Token::Operator(Operation::Multiply) => {
                mult_values.push(current_sum);
                current_sum = 0;
            }
        };
    }
    mult_values.push(current_sum);
    mult_values.iter().product()
}
