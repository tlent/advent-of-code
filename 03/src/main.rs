const INPUT: &str = include_str!("../input.txt");

fn main() {
    let rucksacks: Vec<_> = INPUT.lines().map(|line| Rucksack::new(line)).collect();
    let compartment_sum: u32 = rucksacks
        .iter()
        .map(|rucksack| {
            find_common_item(&rucksack.compartments())
                .unwrap()
                .priority()
        })
        .sum();
    println!("{}", compartment_sum);
    let rucksacks_items: Vec<_> = rucksacks.iter().map(|rucksack| rucksack.items()).collect();
    let group_sum: u32 = rucksacks_items
        .chunks(3)
        .map(|chunk| find_common_item(chunk).unwrap().priority())
        .sum();
    println!("{}", group_sum);
}

fn find_common_item(item_groups: &[&[ItemType]]) -> Option<ItemType> {
    let mut counts = [0; 52];
    for &item_group in item_groups {
        let mut counted = [false; 52];
        for item in item_group {
            let index = (item.priority() - 1) as usize;
            if counted[index] {
                continue;
            }
            counted[index] = true;
            counts[index] += 1;
            if counts[index] == item_groups.len() {
                return Some(*item);
            }
        }
    }
    None
}

#[derive(Debug, Clone, Copy)]
struct ItemType(char);

impl ItemType {
    fn new(c: char) -> Self {
        if !c.is_alphabetic() {
            panic!("invalid item type");
        }
        Self(c)
    }

    fn priority(&self) -> u32 {
        match self.0 {
            'a'..='z' => (self.0 as u32 - 'a' as u32) + 1,
            'A'..='Z' => (self.0 as u32 - 'A' as u32) + 27,
            _ => unreachable!(),
        }
    }
}

struct Rucksack(Vec<ItemType>);

impl Rucksack {
    fn new(s: &str) -> Self {
        let items = s.chars().map(|c| ItemType::new(c)).collect();
        Self(items)
    }

    fn compartments(&self) -> [&[ItemType]; 2] {
        let (a, b) = self.0.split_at(self.0.len() / 2);
        [a, b]
    }

    fn items(&self) -> &[ItemType] {
        &self.0
    }
}
