use advent_of_code_2022::day_07::{self, INPUT};

fn main() {
    let (directory_sizes, root_size) = day_07::parse_input(INPUT);
    let part_one = day_07::part_one(&directory_sizes);
    let part_two = day_07::part_two(&directory_sizes, root_size);
    println!("{part_one}\n{part_two}");
}
