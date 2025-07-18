use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &str = include_str!("../input");

fn main() {
    let graph = parse_input(INPUT);
    let part_one_graph = graph
        .iter()
        .map(|(key, values)| {
            (
                key.clone(),
                values
                    .iter()
                    .map(|(_, color)| color)
                    .cloned()
                    .collect::<Vec<String>>(),
            )
        })
        .collect();
    println!("part one: {}", part_one(&part_one_graph));

    println!("part two: {}", part_two(&graph));
}

fn parse_input(input: &str) -> HashMap<String, Vec<(u32, String)>> {
    let mut graph = HashMap::new();
    for l in input.lines() {
        let mut parts = l.split(" bags contain ");
        let key = String::from(parts.next().unwrap());
        let contents = parts.next().unwrap();
        let value = contents
            .split(", ")
            .filter_map(|c| {
                if c == "no other bags." {
                    return None;
                }
                let mut words = c.split_whitespace();
                let count = words.next().unwrap().parse().unwrap();
                let color = format!("{} {}", words.next().unwrap(), words.next().unwrap());
                Some((count, color))
            })
            .collect();
        graph.insert(key, value);
    }
    graph
}

fn part_one(graph: &HashMap<String, Vec<String>>) -> u32 {
    let mut valid = HashSet::new();
    for color in graph.keys() {
        let mut stack = vec![color];
        while let Some(c) = stack.pop() {
            let adjacent = graph.get(c).unwrap();
            if adjacent.contains(&String::from("shiny gold")) {
                valid.insert(c);
                valid.insert(color);
                break;
            }
            stack.extend(adjacent);
        }
    }
    valid.len() as u32
}

fn part_two(graph: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    let mut total_bags = 0;
    let mut stack = vec!["shiny gold"];
    while let Some(c) = stack.pop() {
        let contents = graph.get(c).unwrap();
        for (count, color) in contents {
            for _ in 0..*count {
                total_bags += 1;
                stack.push(color);
            }
        }
    }
    total_bags
}
