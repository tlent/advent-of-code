const INPUT: &str = include_str!("../input");

fn main() {
    let mut input_lines = INPUT.lines();
    let polymer_template = String::from(input_lines.next().unwrap());
    input_lines.next();
    let mut insertion_rules: PairMap<char> = Default::default();
    for line in input_lines {
        let mut parts = line.split(" -> ");
        let mut pair_chars = parts.next().unwrap().chars();
        let pair = (pair_chars.next().unwrap(), pair_chars.next().unwrap());
        let element = parts.next().unwrap().chars().next().unwrap();
        *insertion_rules.get_mut(&pair) = element;
    }
    let mut char_counts = [0; 26];
    for c in polymer_template.chars() {
        char_counts[char_to_index(c)] += 1;
    }
    let mut pair_counts: PairMap<u64> = Default::default();
    let polymer_chars: Vec<_> = polymer_template.chars().collect();
    for pair in polymer_chars.windows(2) {
        *pair_counts.get_mut(&(pair[0], pair[1])) += 1;
    }
    for step in 1..=40 {
        let prev_pair_counts = pair_counts;
        for a in 'A'..='Z' {
            for b in 'A'..='Z' {
                let count = *prev_pair_counts.get(&(a, b));
                if count > 0 {
                    let new = *insertion_rules.get(&(a, b));
                    *pair_counts.get_mut(&(a, b)) -= count;
                    *pair_counts.get_mut(&(a, new)) += count;
                    *pair_counts.get_mut(&(new, b)) += count;
                    char_counts[char_to_index(new)] += count;
                }
            }
        }
        if step == 10 {
            println!(
                "{}",
                char_counts.iter().max().unwrap()
                    - char_counts.iter().filter(|&&c| c > 0).min().unwrap()
            )
        }
    }
    println!(
        "{}",
        char_counts.iter().max().unwrap() - char_counts.iter().filter(|&&c| c > 0).min().unwrap()
    )
}

#[derive(Debug, Default, Clone, Copy)]
struct PairMap<T>([[T; 26]; 26]);

impl<T> PairMap<T> {
    pub fn get(&self, pair: &(char, char)) -> &T {
        let i = char_to_index(pair.0);
        let j = char_to_index(pair.1);
        &self.0[i][j]
    }

    pub fn get_mut(&mut self, pair: &(char, char)) -> &mut T {
        let i = char_to_index(pair.0);
        let j = char_to_index(pair.1);
        &mut self.0[i][j]
    }
}

fn char_to_index(c: char) -> usize {
    (c as u32 - 'A' as u32) as usize
}
