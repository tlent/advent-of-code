const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut group_sums: Vec<_> = INPUT
        .split("\n\n")
        .map(|s| s.lines().map(|v| v.parse::<u32>().unwrap()).sum())
        .collect();
    group_sums.sort_unstable();

    println!("{}", group_sums.last().unwrap());
    println!("{}", group_sums.iter().rev().take(3).sum::<u32>());
}
