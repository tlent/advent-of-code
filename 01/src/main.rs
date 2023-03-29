const INPUT: &str = include_str!("../input.txt");

use advent_of_code_2022_day_01::{parse_input, part_one, part_two};

fn main() {
    let mut group_sums = parse_input(&INPUT);
    group_sums.sort_unstable();

    println!("{}", part_one(&group_sums));
    println!("{}", part_two(&group_sums));
}
