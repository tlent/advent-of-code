const INPUT: &str = include_str!("../input");

fn main() {
    let initial_fish: Vec<u8> = INPUT
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", part_one(&initial_fish));
    println!("{}", part_two(&initial_fish));
}

fn part_one(fish: &[u8]) -> u64 {
    fish_spawn_dynamic_programming(fish, 80)
}

fn part_two(fish: &[u8]) -> u64 {
    fish_spawn_dynamic_programming(fish, 256)
}

fn fish_spawn_dynamic_programming(fish: &[u8], days: usize) -> u64 {
    let mut fish_counts = vec![0; days + 1];
    let mut spawn_counts = vec![0; days + 1];
    fish_counts[0] = fish.len() as u64;
    for &f in fish {
        for day in (f as usize..=days).step_by(7) {
            spawn_counts[day] += 1;
        }
    }
    for current_day in 1..=days {
        fish_counts[current_day] = fish_counts[current_day - 1] + spawn_counts[current_day - 1];
        for spawn_day in ((current_day + 8)..=days).step_by(7) {
            spawn_counts[spawn_day] += spawn_counts[current_day - 1];
        }
    }
    fish_counts[days]
}

// initial solution that works for part one but is too slow for part two
fn _fish_spawn_sim(fish: &[u8], days: usize) -> u64 {
    let mut fish = fish.to_vec();
    for _ in 0..days {
        let mut new_fish_count = 0;
        for f in fish.iter_mut() {
            if *f == 0 {
                new_fish_count += 1;
                *f = 6;
            } else {
                *f -= 1;
            }
        }
        fish.resize(fish.len() + new_fish_count, 8);
    }
    fish.len() as u64
}
