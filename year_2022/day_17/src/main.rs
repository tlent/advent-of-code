use day_17::{self, INPUT};

fn main() {
    let parse_result = day_17::parse_input(INPUT);
    let part_one = day_17::part_one(&parse_result);
    println!("{part_one}");
    let part_two = day_17::part_two(&parse_result);
    println!("{part_two}");
}
