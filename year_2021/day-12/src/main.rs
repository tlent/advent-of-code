const INPUT: &str = include_str!("../input");

fn main() {
    let graph = Graph::from_input(INPUT);
    println!("{}", part_one(&graph));
    println!("{}", part_two(&graph));
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    adjacency_list: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    label: String,
    id: usize,
    cave_type: CaveType,
}

#[derive(Debug, PartialEq, Eq)]
enum CaveType {
    Small,
    Big,
}

impl Graph {
    fn from_input(s: &str) -> Self {
        let mut nodes = vec![
            Node {
                label: String::from("start"),
                id: 0,
                cave_type: CaveType::Small,
            },
            Node {
                label: String::from("end"),
                id: 1,
                cave_type: CaveType::Small,
            },
        ];
        let mut adjacency_list = vec![vec![]; 2];
        for line in s.lines() {
            let mut parts = line.split('-');
            let from_label = parts.next().unwrap();
            let from_node_id = match nodes.iter().find(|n| n.label == from_label) {
                Some(n) => n.id,
                None => {
                    let n = Node {
                        label: String::from(from_label),
                        id: nodes.len(),
                        cave_type: if from_label.chars().next().unwrap().is_uppercase() {
                            CaveType::Big
                        } else {
                            CaveType::Small
                        },
                    };
                    let id = n.id;
                    nodes.push(n);
                    adjacency_list.push(vec![]);
                    id
                }
            };
            let to_label = parts.next().unwrap();
            let to_node_id = match nodes.iter().find(|n| n.label == to_label) {
                Some(n) => n.id,
                None => {
                    let n = Node {
                        label: String::from(to_label),
                        id: nodes.len(),
                        cave_type: if to_label.chars().next().unwrap().is_uppercase() {
                            CaveType::Big
                        } else {
                            CaveType::Small
                        },
                    };
                    let id = n.id;
                    nodes.push(n);
                    adjacency_list.push(vec![]);
                    id
                }
            };
            adjacency_list[from_node_id].push(to_node_id);
            adjacency_list[to_node_id].push(from_node_id);
        }
        Self {
            nodes,
            adjacency_list,
        }
    }

    fn start_node(&self) -> &Node {
        &self.nodes[0]
    }

    fn end_node(&self) -> &Node {
        &self.nodes[1]
    }
}

fn part_one(graph: &Graph) -> u32 {
    let mut path_count = 0;
    let mut stack = vec![];
    let start_node = graph.start_node();
    let mut visited = vec![false; graph.nodes.len()];
    visited[start_node.id] = true;
    let path = vec![start_node.label.as_str()];
    stack.push((start_node, visited, path));
    while let Some((current_node, visited, path)) = stack.pop() {
        if current_node == graph.end_node() {
            path_count += 1;
            // println!("{}", path.join(","));
            continue;
        }
        let adjacent_nodes = graph.adjacency_list[current_node.id]
            .iter()
            .map(|&id| &graph.nodes[id]);
        for node in adjacent_nodes {
            if node.cave_type == CaveType::Big || !visited[node.id] {
                let mut new_visited = visited.clone();
                new_visited[node.id] = true;
                let mut new_path = path.clone();
                new_path.push(node.label.as_str());
                stack.push((node, new_visited, new_path));
            }
        }
    }
    path_count
}

fn part_two(graph: &Graph) -> u32 {
    let mut path_count = 0;
    let mut stack = vec![];
    let start_node = graph.start_node();
    let mut visited = vec![false; graph.nodes.len()];
    visited[start_node.id] = true;
    let path = vec![start_node.label.as_str()];
    stack.push((start_node, visited, path, false));
    while let Some((current_node, visited, path, used_double_visit)) = stack.pop() {
        if current_node == graph.end_node() {
            path_count += 1;
            // println!("{}", path.join(","));
            continue;
        }
        let adjacent_nodes = graph.adjacency_list[current_node.id]
            .iter()
            .map(|&id| &graph.nodes[id]);
        for node in adjacent_nodes {
            if node.cave_type == CaveType::Big || !visited[node.id] {
                let mut new_visited = visited.clone();
                new_visited[node.id] = true;
                let mut new_path = path.clone();
                new_path.push(node.label.as_str());
                stack.push((node, new_visited, new_path, used_double_visit));
            } else if !used_double_visit && node != start_node {
                let mut new_path = path.clone();
                new_path.push(node.label.as_str());
                stack.push((node, visited.clone(), new_path, true));
            }
        }
    }
    path_count
}
