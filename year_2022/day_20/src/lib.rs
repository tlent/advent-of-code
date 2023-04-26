pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> List {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(mut list: List) -> i32 {
    for i in 0..list.nodes_len() {
        let value = list.node_value(i);
        match value.cmp(&0) {
            std::cmp::Ordering::Greater => list.move_node_forward(i, value as usize),
            std::cmp::Ordering::Less => list.move_node_backward(i, (-value) as usize),
            std::cmp::Ordering::Equal => {}
        }
    }
    let mut iter = list.iter();
    [
        iter.nth(1000).unwrap(),
        iter.nth(999).unwrap(),
        iter.nth(999).unwrap(),
    ]
    .into_iter()
    .sum()
}

pub fn part_two() {
    todo!()
}

/// A circular doubly-linked list.
#[derive(Debug)]
pub struct List {
    nodes: Vec<Node>,
    head: usize,
}

#[derive(Debug)]
struct Node {
    value: i32,
    previous: usize,
    next: usize,
}

impl List {
    fn iter(&self) -> Iter {
        Iter {
            nodes: &self.nodes,
            cursor: self.head,
        }
    }

    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    fn node_value(&self, node_index: usize) -> i32 {
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
}

impl FromIterator<i32> for List {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
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
    type Item = &'a i32;

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
        todo!()
    }

    #[test]
    fn test_part_two() {
        todo!()
    }
}
