const INPUT: &str = include_str!("../input.txt");

fn main() {
    let groups: Vec<Vec<u32>> = INPUT
        .split("\n\n")
        .map(|s| s.lines().map(|v| v.parse().unwrap()).collect())
        .collect();
    let mut group_sums: Vec<u32> = groups.iter().map(|group| group.iter().sum()).collect();
    group_sums.sort_unstable();
    let max_sum = group_sums.last().unwrap();
    println!("{}", max_sum);
    let top_three_sum: u32 = group_sums[group_sums.len() - 3..].iter().sum();
    println!("{}", top_three_sum);
}
