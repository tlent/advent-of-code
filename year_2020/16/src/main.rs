use std::collections::HashSet;
use std::ops::RangeInclusive;

static INPUT: &str = include_str!("../input");

fn main() {
    let (rules, my_ticket, nearby_tickets) = parse_input(INPUT);
    println!("part one: {}", part_one(&rules, &nearby_tickets));
    println!(
        "part two: {}",
        part_two(&rules, &my_ticket, &nearby_tickets)
    );
}

#[derive(Debug)]
struct Rule {
    field_name: String,
    valid_ranges: [RangeInclusive<u32>; 2],
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>) {
    let mut parts = input.split("\n\n");
    let rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split(": ");
            let field_name = String::from(parts.next().unwrap());
            let mut range_strings = parts.next().unwrap().split(" or ");
            let mut valid_ranges = [0..=0, 0..=0];
            for range in valid_ranges.iter_mut() {
                let mut values = range_strings.next().unwrap().split("-");
                let min = values.next().unwrap().parse().unwrap();
                let max = values.next().unwrap().parse().unwrap();
                *range = min..=max;
            }
            Rule {
                field_name,
                valid_ranges,
            }
        })
        .collect();
    let my_ticket = parts
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let nearby_tickets = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();
    (rules, my_ticket, nearby_tickets)
}

fn part_one(rules: &[Rule], nearby_tickets: &[Vec<u32>]) -> u32 {
    let values: Vec<_> = nearby_tickets.iter().flatten().copied().collect();
    values
        .iter()
        .filter(|&&v| {
            !rules
                .iter()
                .any(|r| r.valid_ranges.iter().any(|range| range.contains(&v)))
        })
        .sum()
}

fn part_two(rules: &[Rule], my_ticket: &[u32], nearby_tickets: &[Vec<u32>]) -> u64 {
    let my_ticket = my_ticket.to_vec();
    let mut valid_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|t| {
            t.iter().all(|&v| {
                rules
                    .iter()
                    .any(|r| r.valid_ranges.iter().any(|range| range.contains(&v)))
            })
        })
        .collect();
    valid_tickets.push(&my_ticket);
    let mut possible_fields_by_position: Vec<HashSet<_>> = (0..my_ticket.len())
        .map(|i| {
            rules
                .iter()
                .filter(|r| {
                    valid_tickets
                        .iter()
                        .map(|t| t[i])
                        .all(|v| r.valid_ranges.iter().any(|range| range.contains(&v)))
                })
                .map(|r| &r.field_name)
                .collect()
        })
        .collect();
    let mut field_by_position = vec![None; my_ticket.len()];
    let mut used_fields = HashSet::new();
    while used_fields.len() < my_ticket.len() {
        for (i, fields) in possible_fields_by_position.iter_mut().enumerate() {
            *fields = fields.difference(&used_fields).cloned().collect();
            if fields.len() == 1 {
                let field = fields.drain().nth(0).unwrap();
                field_by_position[i] = Some(field);
                used_fields.insert(field);
            }
        }
    }
    field_by_position
        .iter()
        .enumerate()
        .filter(|(_, field)| field.unwrap().starts_with("departure"))
        .map(|(position, _)| my_ticket[position] as u64)
        .product()
}
