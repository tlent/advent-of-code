const INPUT: &str = include_str!("../input.txt");

struct Item(u8);

impl Item {
    fn from_ascii_byte(byte: u8) -> Item {
        if !byte.is_ascii_alphabetic() {
            panic!("{} is invalid", byte);
        }
        Item(byte)
    }

    fn priority(&self) -> u32 {
        let c = self.0;
        (match c {
            b'a'..=b'z' => 1 + c - b'a',
            b'A'..=b'Z' => 27 + c - b'A',
            _ => unreachable!(),
        }) as u32
    }
}

struct ItemSet([bool; 52]);

impl ItemSet {
    fn shared_item(sets: &[ItemSet]) -> Option<Item> {
        for index in 0..52 {
            if sets.iter().all(|set| set.0[index]) {
                return Some(Self::index_to_item(index));
            }
        }
        None
    }

    fn item_to_index(item: Item) -> usize {
        (item.priority() - 1) as usize
    }

    fn index_to_item(index: usize) -> Item {
        let index = index as u8;
        Item(match index {
            0..=25 => index + b'a',
            26..=51 => index - 26 + b'A',
            _ => unreachable!(),
        })
    }
}

impl FromIterator<Item> for ItemSet {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> ItemSet {
        let mut item_set = ItemSet([false; 52]);
        for index in iter.into_iter().map(ItemSet::item_to_index) {
            item_set.0[index] = true;
        }
        item_set
    }
}

fn main() {
    let lines: Vec<_> = INPUT.lines().collect();
    let part_one_solution = lines
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left_items: ItemSet = left.bytes().map(Item::from_ascii_byte).collect();
            let right_items: ItemSet = right.bytes().map(Item::from_ascii_byte).collect();
            ItemSet::shared_item(&[left_items, right_items])
                .unwrap()
                .priority()
        })
        .sum::<u32>();
    println!("{}", part_one_solution);
    let part_two_solution = lines
        .chunks_exact(3)
        .map(|chunk_lines| {
            let item_sets: Vec<ItemSet> = chunk_lines
                .iter()
                .map(|line| line.bytes().map(Item::from_ascii_byte).collect())
                .collect();
            ItemSet::shared_item(&item_sets).unwrap().priority()
        })
        .sum::<u32>();
    println!("{}", part_two_solution);
}
