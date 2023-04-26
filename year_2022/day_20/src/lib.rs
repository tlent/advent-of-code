pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> List {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one() -> () {
    todo!()
}

pub fn part_two() -> () {
    todo!()
}

/// A circular doubly-linked list.
pub struct List {
    nodes: Vec<Node>,
    head: usize,
}

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
