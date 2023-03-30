use advent_of_code_2022::day_07::{self, INPUT};

fn main() {
    let (directory_sizes, root_size) = day_07::parse_input(INPUT);
    println!("{}", day_07::part_one(&directory_sizes));
    println!("{}", day_07::part_two(&directory_sizes, root_size));
}
