pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> List {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(mut list: List) -> i64 {
    mix(&mut list);
    let mut iter = list.iter();
    [1000, 999, 999]
        .into_iter()
        .map(|i| iter.nth(i).unwrap())
        .sum()
}

pub fn part_two(mut list: List) -> i64 {
    const MULTIPLIER: i64 = 811_589_153;
    for node in list.nodes.iter_mut() {
        node.value *= MULTIPLIER;
    }
    for _ in 0..10 {
        mix(&mut list);
    }
    let mut iter = list.iter();
    [1000, 999, 999]
        .into_iter()
        .map(|i| iter.nth(i).unwrap())
        .sum()
}

fn mix(list: &mut List) {
    for i in 0..list.nodes_len() {
        let value = list.get_node_value(i);
        // use len - 1 because the moved node has been removed
        let steps = value.unsigned_abs() as usize % (list.nodes_len() - 1);
        if steps == 0 {
            continue;
        } else if value > 0 {
            list.move_node_forward(i, steps);
        } else if value < 0 {
            list.move_node_backward(i, steps)
        }
    }
}

/// A circular doubly-linked list.
#[derive(Debug, Clone)]
pub struct List {
    nodes: Vec<Node>,
    head: usize,
}

#[derive(Debug, Clone)]
struct Node {
    value: i64,
    previous: usize,
    next: usize,
}

impl List {
    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    fn get_node_value(&self, node_index: usize) -> i64 {
        self.nodes[node_index].value
    }

    fn move_node_forward(&mut self, node_index: usize, steps: usize) {
        let prev_node = self.nodes[node_index].previous;
        let next_node = self.nodes[node_index].next;
        self.nodes[prev_node].next = next_node;
        self.nodes[next_node].previous = prev_node;
        let mut cursor = node_index;
        for _ in 0..steps {
            cursor = self.nodes[cursor].next;
        }
        let cursor_next = self.nodes[cursor].next;
        self.nodes[node_index].previous = cursor;
        self.nodes[node_index].next = cursor_next;
        self.nodes[cursor].next = node_index;
        self.nodes[cursor_next].previous = node_index;
    }

    fn move_node_backward(&mut self, node_index: usize, steps: usize) {
        let prev_node = self.nodes[node_index].previous;
        let next_node = self.nodes[node_index].next;
        self.nodes[prev_node].next = next_node;
        self.nodes[next_node].previous = prev_node;
        let mut cursor = node_index;
        for _ in 0..=steps {
            cursor = self.nodes[cursor].previous;
        }
        let cursor_next = self.nodes[cursor].next;
        self.nodes[node_index].previous = cursor;
        self.nodes[node_index].next = cursor_next;
        self.nodes[cursor].next = node_index;
        self.nodes[cursor_next].previous = node_index;
    }

    fn iter(&self) -> Iter {
        Iter {
            nodes: &self.nodes,
            cursor: self.head,
        }
    }
}

impl FromIterator<i64> for List {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let mut nodes = iter
            .into_iter()
            .enumerate()
            .map(|(i, value)| Node {
                value,
                previous: i.saturating_sub(1),
                next: i.saturating_add(1),
            })
            .collect::<Vec<_>>();
        let last_index = nodes.len() - 1;
        nodes[0].previous = last_index;
        nodes[last_index].next = 0;
        let head = nodes.iter().position(|n| n.value == 0).unwrap();
        Self { nodes, head }
    }
}

pub struct Iter<'a> {
    nodes: &'a Vec<Node>,
    cursor: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = &self.nodes[self.cursor].value;
        self.cursor = self.nodes[self.cursor].next;
        Some(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let list = parse_input(INPUT);
        assert_eq!(part_one(list), 15297);
    }

    #[test]
    fn test_part_two() {
        let list = parse_input(INPUT);
        assert_eq!(part_two(list), 2_897_373_276_210);
    }
}
