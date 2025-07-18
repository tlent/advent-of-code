const INPUT: &str = include_str!("../input");

fn main() {
    let numbers: Vec<_> = INPUT
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect();
    let max = *numbers.iter().max().unwrap();
    let bit_count = bit_count(max);
    println!("{}", part_one(&numbers, bit_count));
    println!("{}", part_two(&numbers, bit_count));
}

fn bit_count(value: u32) -> u32 {
    let mut count = 1;
    let mut m = 1;
    while value >= m {
        count += 1;
        m <<= 1;
    }
    count - 1
}

fn part_one(numbers: &[u32], bit_count: u32) -> u32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let mut mask = 1;
    for _ in 0..bit_count {
        let mut one_count = 0;
        for x in numbers {
            if x & mask > 0 {
                one_count += 1;
            }
        }
        if one_count > numbers.len() / 2 {
            gamma_rate |= mask;
        } else {
            epsilon_rate |= mask;
        }
        mask <<= 1;
    }
    gamma_rate * epsilon_rate
}

fn part_two(numbers: &[u32], bit_count: u32) -> u32 {
    let oxygen_generator_rating = find_rating(numbers, bit_count, |ones, zeroes| {
        if ones.len() >= zeroes.len() {
            ones
        } else {
            zeroes
        }
    });
    let co2_scrubber_rating = find_rating(numbers, bit_count, |ones, zeroes| {
        if zeroes.len() <= ones.len() {
            zeroes
        } else {
            ones
        }
    });
    oxygen_generator_rating * co2_scrubber_rating
}

fn find_rating<F>(numbers: &[u32], bit_count: u32, f: F) -> u32
where
    F: Fn(Vec<u32>, Vec<u32>) -> Vec<u32>,
{
    let mut numbers = numbers.to_vec();
    for bit in (0..bit_count).rev() {
        let mask = 1 << bit;
        let ones: Vec<_> = numbers.iter().filter(|&v| v & mask > 0).copied().collect();
        let zeroes: Vec<_> = numbers.iter().filter(|&v| v & mask == 0).copied().collect();
        numbers = f(ones, zeroes);
        if numbers.len() == 1 {
            return numbers[0];
        }
    }
    panic!("no rating found");
}
