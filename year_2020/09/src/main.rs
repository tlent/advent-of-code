static INPUT: &str = include_str!("../input");

fn main() {
    let numbers: Vec<u64> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    let invalid_number = part_one(&numbers);
    println!("part one: {}", invalid_number);
    println!("part two: {}", part_two(&numbers, invalid_number));
}

fn part_one(numbers: &[u64]) -> u64 {
    for window in numbers.windows(26) {
        let target = window[25];
        let mut is_valid = false;
        for a in &window[..25] {
            for b in &window[..25] {
                if a + b == target {
                    is_valid = true;
                }
            }
        }
        if !is_valid {
            return target;
        }
    }
    panic!("no solution found");
}

fn part_two(numbers: &[u64], target: u64) -> u64 {
    let mut start = 0;
    let mut end = 2;
    let mut sum: u64 = numbers[..end].iter().sum();
    while sum != target {
        if sum < target {
            sum += numbers[end];
            end += 1;
        } else {
            sum -= numbers[start];
            start += 1;
        }
        while end - start < 2 {
            sum += numbers[end];
            end += 1;
        }
    }
    let min = numbers[start..end].iter().min().unwrap();
    let max = numbers[start..end].iter().max().unwrap();
    min + max
}
