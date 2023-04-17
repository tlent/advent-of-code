use day_16::{self, INPUT};

fn main() {
    let valves = day_16::parser::parse(INPUT).unwrap();
    let (processed_valves, initial_distances) = day_16::preprocess(valves);
    let part_one = day_16::part_one(&processed_valves, &initial_distances);
    println!("{part_one}");
    let part_two = day_16::part_two(&processed_valves, &initial_distances);
    println!("{part_two}");
}
