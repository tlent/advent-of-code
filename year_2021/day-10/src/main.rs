const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

fn part_one(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let (expected, points) = match c {
                        ')' => ('(', 3),
                        ']' => ('[', 57),
                        '}' => ('{', 1197),
                        '>' => ('<', 25137),
                        _ => unreachable!(),
                    };
                    if stack.pop() != Some(expected) {
                        score += points;
                        break;
                    }
                }
                _ => panic!("unexpected char"),
            }
        }
    }
    score
}

fn part_two(input: &str) -> u64 {
    let mut scores = vec![];
    'line: for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let expected = match c {
                        ')' => '(',
                        ']' => '[',
                        '}' => '{',
                        '>' => '<',
                        _ => unreachable!(),
                    };
                    if stack.pop() != Some(expected) {
                        continue 'line;
                    }
                }
                _ => panic!("unexpected char"),
            }
        }
        let mut score = 0;
        while let Some(c) = stack.pop() {
            score = score * 5
                + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
        }
        scores.push(score);
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}
