pub const INPUT: &str = include_str!("../input.txt");

type ParsedInput = Vec<Game>;
type Output = u32;

pub struct Game {
    id: u32,
    samples: Vec<Sample>,
}

pub struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let id = left.split_once(' ').unwrap().1.parse().unwrap();
            let samples = right
                .split("; ")
                .map(|round| {
                    let mut sample = Sample {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    for s in round.split(", ") {
                        let (count, color) = s.split_once(' ').unwrap();
                        let count: u32 = count.parse().unwrap();
                        match color {
                            "red" => sample.red += count,
                            "green" => sample.green += count,
                            "blue" => sample.blue += count,
                            _ => panic!("invalid color"),
                        }
                    }
                    sample
                })
                .collect();
            Game { id, samples }
        })
        .collect()
}

pub fn part_one(games: &ParsedInput) -> Output {
    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 13;
    const BLUE_LIMIT: u32 = 14;
    games
        .iter()
        .filter(|game| {
            game.samples.iter().all(|sample| {
                sample.red <= RED_LIMIT && sample.green <= GREEN_LIMIT && sample.blue <= BLUE_LIMIT
            })
        })
        .map(|game| game.id)
        .sum()
}

pub fn part_two(games: &ParsedInput) -> Output {
    games
        .iter()
        .map(|game| {
            let max_red = game.samples.iter().map(|sample| sample.red).max().unwrap();
            let max_green = game
                .samples
                .iter()
                .map(|sample| sample.green)
                .max()
                .unwrap();
            let max_blue = game.samples.iter().map(|sample| sample.blue).max().unwrap();
            max_red * max_green * max_blue
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let games = parse_input(INPUT);
        assert_eq!(part_one(&games), 3_059);
    }

    #[test]
    fn test_part_two() {
        let games = parse_input(INPUT);
        assert_eq!(part_two(&games), 65_371);
    }
}
