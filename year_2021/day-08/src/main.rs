const INPUT: &str = include_str!("../input");

fn main() {
    let entries: Vec<_> = INPUT.lines().map(Entry::from_str).collect();
    println!("{}", part_one(&entries));
    println!("{}", part_two(&entries));
}

struct Entry {
    signal_patterns: [String; 10],
    output_values: [String; 4],
}

impl Entry {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split(" | ");
        let mut signal_patterns: [String; 10] = Default::default();
        for (i, s) in parts.next().unwrap().split(' ').enumerate() {
            signal_patterns[i] += s;
        }
        let mut output_values: [String; 4] = Default::default();
        for (i, s) in parts.next().unwrap().split(' ').enumerate() {
            output_values[i] += s;
        }
        Self {
            signal_patterns,
            output_values,
        }
    }

    fn solve_value(&self) -> u32 {
        let one = self.signal_patterns.iter().find(|p| p.len() == 2).unwrap();
        let four = self.signal_patterns.iter().find(|p| p.len() == 4).unwrap();
        let seven = self.signal_patterns.iter().find(|p| p.len() == 3).unwrap();
        let eight = self.signal_patterns.iter().find(|p| p.len() == 7).unwrap();
        let three = self
            .signal_patterns
            .iter()
            .find(|p| p.len() == 5 && overlap_count(p, one) == 2)
            .unwrap();
        let two = self
            .signal_patterns
            .iter()
            .find(|p| p.len() == 5 && overlap_count(p, four) == 2)
            .unwrap();
        let five = self
            .signal_patterns
            .iter()
            .find(|p| p.len() == 5 && overlap_count(p, one) == 1 && overlap_count(p, four) == 3)
            .unwrap();
        let six = self
            .signal_patterns
            .iter()
            .find(|p| p.len() == 6 && overlap_count(p, one) == 1)
            .unwrap();
        let nine = self
            .signal_patterns
            .iter()
            .find(|p| p.len() == 6 && overlap_count(p, four) == 4)
            .unwrap();
        let zero = self
            .signal_patterns
            .iter()
            .find(|p| p.len() == 6 && overlap_count(p, one) == 2 && overlap_count(p, four) == 3)
            .unwrap();
        let digit_patterns = [zero, one, two, three, four, five, six, seven, eight, nine];
        self.output_values
            .iter()
            .map(|v| {
                digit_patterns
                    .iter()
                    .position(|&p| v.len() == p.len() && overlap_count(p, v) == v.len() as u8)
                    .unwrap() as u32
            })
            .fold(0, |v, d| v * 10 + d)
    }
}

fn overlap_count(a: &str, b: &str) -> u8 {
    a.chars().filter(|&c| b.contains(c)).count() as u8
}

fn part_one(entries: &[Entry]) -> u32 {
    entries
        .iter()
        .map(|e| {
            e.output_values
                .iter()
                .filter(|s| [2, 3, 4, 7].contains(&s.len()))
                .count() as u32
        })
        .sum()
}

fn part_two(entries: &[Entry]) -> u32 {
    entries.iter().map(|e| e.solve_value()).sum()
}
