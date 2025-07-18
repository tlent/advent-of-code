const INPUT: &str = include_str!("../input");

fn main() {
    let depths = parse_input(INPUT);
    println!("{}", part_one(&depths));
    println!("{}", part_two(&depths));
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_one(depths: &[u32]) -> u32 {
    let mut depths = depths.iter();
    let mut prev = depths.next().unwrap();
    let mut increase_count = 0;
    for d in depths {
        if d > prev {
            increase_count += 1;
        }
        prev = d;
    }
    increase_count
}

fn part_two(depths: &[u32]) -> u32 {
    let mut new_depth = depths.iter();
    let mut prev_three_sum =
        new_depth.next().unwrap() + new_depth.next().unwrap() + new_depth.next().unwrap();
    let mut increase_count = 0;
    for (prev, new) in depths.iter().zip(new_depth) {
        let new_three_sum = prev_three_sum - prev + new;
        if new_three_sum > prev_three_sum {
            increase_count += 1;
        }
        prev_three_sum = new_three_sum;
    }
    increase_count
}
