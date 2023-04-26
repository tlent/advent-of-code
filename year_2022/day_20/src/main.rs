use day_20::{self, INPUT};

fn main() {
    let parse_result = day_20::parse_input(INPUT);
    let part_one = day_20::part_one(&parse_result);
    println!("{part_one}");
    let part_two = day_20::part_two(&parse_result);
    println!("{part_two}");
}
