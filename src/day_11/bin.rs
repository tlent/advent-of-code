use advent_of_code_2022::day_11::{self, INPUT};

fn main() {
    let monkeys = day_11::parse_input(INPUT);
    let part_one = day_11::part_one(&monkeys);
    let part_two = day_11::part_two(&monkeys);
    println!("{part_one}\n{part_two}");
}
