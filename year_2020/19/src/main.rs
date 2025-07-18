use std::collections::HashSet;
use std::rc::Rc;

static INPUT: &str = include_str!("../input");

fn main() {
    let (rules, messages) = parse_input(INPUT);
    let part_one = messages
        .iter()
        .filter(|m| {
            let (matches, rest) = rules[0].matches(m);
            matches && rest.is_empty()
        })
        .count();
    println!("part one: {}", part_one);

    let part_two = messages
        .iter()
        .filter(|m| {
            let mut s = m.as_str();
            let mut a = 0;
            while let (true, rest) = rules[42].matches(s) {
                a += 1;
                s = rest;
            }
            let mut b = 0;
            while let (true, rest) = rules[31].matches(s) {
                b += 1;
                s = rest;
            }
            b > 0 && a > b && s.is_empty()
        })
        .count();
    println!("part two: {}", part_two);
}

trait Rule {
    fn matches<'a>(&self, s: &'a str) -> (bool, &'a str);
    fn to_regex(&self) -> String;
    fn id(&self) -> u32;
}

struct SubruleRule {
    id: u32,
    subrule_sets: Vec<Vec<Rc<dyn Rule>>>,
}

impl Rule for SubruleRule {
    fn matches<'a>(&self, s: &'a str) -> (bool, &'a str) {
        for rule_set in &self.subrule_sets {
            let mut matches_rule_set = true;
            let mut v = s;
            for rule in rule_set {
                let (matches, rest) = rule.matches(v);
                if !matches {
                    matches_rule_set = false;
                    break;
                }
                v = rest;
            }
            if matches_rule_set {
                return (true, v);
            }
        }
        (false, s)
    }

    fn to_regex(&self) -> String {
        let mut set_regexes: Vec<_> = self
            .subrule_sets
            .iter()
            .map(|set| {
                set.iter()
                    .map(|rule| rule.to_regex())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect();
        if set_regexes.len() > 1 {
            format!("({})", set_regexes.join("|"))
        } else {
            set_regexes.pop().unwrap()
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
struct CharacterRule {
    id: u32,
    character: char,
}

impl Rule for CharacterRule {
    fn matches<'a>(&self, s: &'a str) -> (bool, &'a str) {
        if s.starts_with(self.character) {
            (true, &s[1..])
        } else {
            (false, s)
        }
    }

    fn to_regex(&self) -> String {
        self.character.to_string()
    }

    fn id(&self) -> u32 {
        self.id
    }
}

fn parse_input(input: &str) -> (Vec<Rc<dyn Rule>>, Vec<String>) {
    let mut parts = input.split("\n\n");
    let rule_lines = parts.next().unwrap().lines();
    let rule_count = rule_lines.clone().count();
    let mut rules: Vec<Option<Rc<dyn Rule>>> = vec![None; rule_count];
    let mut completed_rules = HashSet::new();
    let mut blocked_rules = vec![];
    for l in rule_lines {
        let mut rule_parts = l.split(": ");
        let id = rule_parts.next().unwrap().parse().unwrap();
        let rest = rule_parts.next().unwrap();
        if rest.starts_with('"') {
            rules[id as usize] = Some(Rc::new(CharacterRule {
                id,
                character: rest.chars().nth(1).unwrap(),
            }));
            completed_rules.insert(id);
        } else {
            let subrule_sets_by_id: Vec<Vec<u32>> = rest
                .split(" | ")
                .map(|set| set.split(' ').map(|v| v.parse().unwrap()).collect())
                .collect();
            let dependencies: HashSet<_> = subrule_sets_by_id.iter().flatten().cloned().collect();
            blocked_rules.push((id, subrule_sets_by_id, dependencies));
        }
    }
    while !blocked_rules.is_empty() {
        let mut remaining_blocked_rules = vec![];
        for (id, subrule_sets_by_id, dependencies) in blocked_rules {
            let dependencies: HashSet<_> =
                dependencies.difference(&completed_rules).cloned().collect();
            if dependencies.is_empty() {
                let subrule_sets = subrule_sets_by_id
                    .into_iter()
                    .map(|set| {
                        set.into_iter()
                            .map(|id| Rc::clone(rules[id as usize].as_ref().unwrap()))
                            .collect()
                    })
                    .collect();
                rules[id as usize] = Some(Rc::new(SubruleRule { id, subrule_sets }));
                completed_rules.insert(id);
            } else {
                remaining_blocked_rules.push((id, subrule_sets_by_id, dependencies));
            }
        }
        blocked_rules = remaining_blocked_rules;
    }
    let rules = rules.into_iter().map(Option::unwrap).collect();
    let messages = parts.next().unwrap().lines().map(String::from).collect();
    (rules, messages)
}
