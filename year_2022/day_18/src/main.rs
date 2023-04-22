use day_18::{self, INPUT};

fn main() {
    let parse_result = day_18::parse_input(INPUT);
    let part_one = day_18::part_one(&parse_result);
    println!("{part_one}");
    let part_two = day_18::part_two(&parse_result);
    println!("{part_two}");
}
