use advent_of_code_2022::day_03::{self, INPUT};

fn main() {
    let (part_one_sets, part_two_sets) = day_03::parse_input(INPUT);
    println!("{}", day_03::part_one(&part_one_sets));
    println!("{}", day_03::part_two(&part_two_sets));
}
