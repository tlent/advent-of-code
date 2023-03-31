use advent_of_code_2022::day_01::{self, INPUT};

fn main() {
    let group_sums = day_01::parse_input(INPUT);
    let (part_one, part_two) = day_01::solve(group_sums);
    println!("{}", part_one);
    println!("{}", part_two);
}
