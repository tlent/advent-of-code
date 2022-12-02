const INPUT: &str = include_str!("../input.txt");

fn main() {
    let inventories = parse_input(&INPUT);
    println!("{}", max_calories(&inventories));
    println!("{}", three_max_sum(&inventories));
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut inventories = vec![];
    let mut inventory = vec![];
    for line in input.lines() {
        if line.is_empty() {
            inventories.push(inventory);
            inventory = vec![];
        } else {
            let value = line.parse().unwrap();
            inventory.push(value);
        }
    }
    inventories
}

fn max_calories(inventories: &[Vec<u32>]) -> u32 {
    inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .max()
        .unwrap()
}

fn three_max_sum(inventories: &[Vec<u32>]) -> u32 {
    let mut sums: Vec<_> = inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect();
    sums.sort_unstable();
    sums[sums.len() - 3..].iter().sum()
}
