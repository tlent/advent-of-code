const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut group_sums = parse_input(&INPUT);
    group_sums.sort_unstable();

    println!("{}", part_one(&group_sums));
    println!("{}", part_two(&group_sums));
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|v| v.parse::<u32>().unwrap()).sum())
        .collect()
}

fn part_one(sorted_group_sums: &[u32]) -> u32 {
    *sorted_group_sums.last().unwrap()
}

fn part_two(sorted_group_sums: &[u32]) -> u32 {
    sorted_group_sums.iter().rev().take(3).sum::<u32>()
}
