pub const INPUT: &str = include_str!("../input.txt");

pub struct Race {
    time: u64,
    distance: u64,
}

pub fn parse_input(input: &str) -> (Vec<Race>, Race) {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();
    let times = time_line[12..]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap());
    let distances = distance_line[12..]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap());
    let part_one_races = times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();
    let part_two_race = Race {
        time: parse_digits(&time_line[12..]),
        distance: parse_digits(&distance_line[12..]),
    };
    (part_one_races, part_two_race)
}

fn parse_digits(s: &str) -> u64 {
    let mut value = 0;
    for b in s.bytes() {
        if b.is_ascii_digit() {
            value = value * 10 + (b - b'0') as u64;
        }
    }
    value
}

pub fn part_one(races: &[Race]) -> u64 {
    races
        .iter()
        .map(|race| {
            (0..race.time)
                .filter(|t| t * (race.time - t) > race.distance)
                .count() as u64
        })
        .product()
}

pub fn part_two(race: &Race) -> u64 {
    (0..race.time)
        .filter(|t| t * (race.time - t) > race.distance)
        .count() as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let (part_one_races, _) = parse_input(INPUT);
        assert_eq!(part_one(&part_one_races), 1_413_720);
    }

    #[test]
    fn test_part_two() {
        let (_, part_two_race) = parse_input(INPUT);
        assert_eq!(part_two(&part_two_race), 30_565_288);
    }
}
