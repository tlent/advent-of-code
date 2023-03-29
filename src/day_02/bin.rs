use advent_of_code_2022::day_02::{self, INPUT};

fn main() {
    let (part_one_rounds, part_two_rounds) = day_02::parse_input(INPUT);
    println!("{}", day_02::part_one(&part_one_rounds));
    println!("{}", day_02::part_two(&part_two_rounds));
}
