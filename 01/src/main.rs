const INPUT: &str = include_str!("../input.txt");

fn main() {
    let calorie_totals = parse_input(&INPUT);
    let max_calorie_total = calorie_totals.iter().max().unwrap();
    println!("{}", max_calorie_total);
    let three_largest_sum: u32 = three_largest(&calorie_totals).iter().sum();
    println!("{}", three_largest_sum);
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut totals = vec![];
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            totals.push(total);
            total = 0;
        } else {
            let value: u32 = line.parse().unwrap();
            total += value;
        }
    }
    totals
}

fn three_largest(values: &[u32]) -> [u32; 3] {
    let mut values = values.to_vec();
    values.sort_unstable();
    values[values.len() - 3..].try_into().unwrap()
}
