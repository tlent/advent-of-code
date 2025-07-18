static INPUT: &str = include_str!("../input");

fn main() {
    let input = parse_input(INPUT);
    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

struct Rule {
    min_count: u32,
    max_count: u32,
    required_character: char,
}

impl Rule {
    fn allows(&self, password: &str) -> bool {
        let count = password.matches(self.required_character).count() as u32;
        count >= self.min_count && count <= self.max_count
    }

    fn allows_part_two(&self, password: &str) -> bool {
        let char_a = password.chars().nth(self.min_count as usize - 1).unwrap();
        let char_b = password.chars().nth(self.max_count as usize - 1).unwrap();
        (char_a == self.required_character) != (char_b == self.required_character)
    }
}

fn parse_input(input: &str) -> Vec<(Rule, String)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let mut rule_parts = parts.next().unwrap().split('-');
            let min_count = rule_parts.next().unwrap().parse().unwrap();
            let max_count = rule_parts.next().unwrap().parse().unwrap();
            let required_character = parts.next().unwrap().chars().nth(0).unwrap();
            let rule = Rule {
                min_count,
                max_count,
                required_character,
            };
            let password = String::from(parts.next().unwrap());
            (rule, password)
        })
        .collect()
}

fn part_one(input: &[(Rule, String)]) -> u32 {
    input.iter().filter(|&(r, s)| r.allows(s)).count() as u32
}

fn part_two(input: &[(Rule, String)]) -> u32 {
    input.iter().filter(|&(r, s)| r.allows_part_two(s)).count() as u32
}
