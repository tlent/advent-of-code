use advent_of_code_2022::day_01::{self, INPUT};

fn main() {
    let mut group_sums = day_01::parse_input(&INPUT);
    group_sums.sort_unstable();

    println!("{}", day_01::part_one(&group_sums));
    println!("{}", day_01::part_two(&group_sums));
}
