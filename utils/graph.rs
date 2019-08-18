
trait GraphNode {
    fn get_adjacent_node_indices(&self) -> &Vec<usize>;
    fn add_adjacent_node(&mut self, index: usize);
}

fn scan_adjacent_nodes<T>(cin: &mut CharacterInput, edge_count: usize, node_list: &mut Vec<T>)
where
    T: GraphNode,
{
    for _i in 0..edge_count {
        let index_a = cin.scan_u(SP) as usize - 1;
        let index_b = cin.scan_u(LF) as usize - 1;
        node_list[index_a].add_adjacent_node(index_b);
        node_list[index_b].add_adjacent_node(index_a);
    }
}

fn depth_first_search<T, F>(
    nodes: &mut Vec<T>,
    n: usize,
    start_index: usize,
    visit_callback: &mut F, // (node_index, previous_node_index) => true_if_found
) -> Option<usize>
where
    T: GraphNode,
    F: FnMut(usize, Option<usize>) -> bool,
{
    let mut node_visited_indicator_table = vec![false; n];

    let mut frontier_index_queue: Vec<(usize, Option<usize>)> = Vec::with_capacity(n / 2);
    frontier_index_queue.push((start_index, None));

    loop {
        let next = frontier_index_queue.pop();

        match next {
            Some((node_index, parent_node_index)) => {
                if visit_callback(node_index, parent_node_index) {
                    return Some(node_index);
                }

                for neighbor in nodes[node_index]
                    .get_adjacent_node_indices()
                    .iter()
                    .map(|rf| *rf)
                    .filter(|neighbor| node_visited_indicator_table[*neighbor] == false)
                {
                    frontier_index_queue.push((neighbor, Some(node_index))); // for_each() is not supported by v1.15.1
                }

                node_visited_indicator_table[node_index] = true;
            }
            None => break,
        }
    }

    None
}

use std::collections::VecDeque;

fn breadth_first_search<T, F>(
    nodes: &mut Vec<T>,
    n: usize,
    start_index: usize,
    visit_callback: &mut F, // (node_index, previous_node_index) => true_if_found
) -> Option<usize>
where
    T: GraphNode,
    F: FnMut(usize, Option<usize>) -> bool,
{
    let mut node_visited_indicator_table = vec![false; n];

    let mut frontier_index_queue: VecDeque<(usize, Option<usize>)> = VecDeque::with_capacity(n / 2);
    frontier_index_queue.push_back((start_index, None));

    loop {
        let next = frontier_index_queue.pop_front();

        match next {
            Some((node_index, parent_node_index)) => {
                if visit_callback(node_index, parent_node_index) {
                    return Some(node_index);
                }

                for neighbor in nodes[node_index]
                    .get_adjacent_node_indices()
                    .iter()
                    .map(|rf| *rf)
                    .filter(|neighbor| node_visited_indicator_table[*neighbor] == false)
                {
                    frontier_index_queue.push_back((neighbor, Some(node_index))); // for_each() is not supported by v1.15.1
                }

                node_visited_indicator_table[node_index] = true;
            }
            None => break,
        }
    }

    None
}

// Sample
struct Node {
    adjacent_node_indices: Vec<usize>,
}

impl Node {
    fn new_list(length: usize) -> Vec<Node> {
        (0..length)
            .map(|_| Node {
                adjacent_node_indices: Vec::new(),
            })
            .collect()
    }

    fn new_list_with_capacity(length: usize, capacity: usize) -> Vec<Node> {
        (0..length)
            .map(|_| Node {
                adjacent_node_indices: Vec::with_capacity(capacity),
            })
            .collect()
    }
}

impl GraphNode for Node {
    fn get_adjacent_node_indices(&self) -> &Vec<usize> {
        &self.adjacent_node_indices
    }

    fn add_adjacent_node(&mut self, index: usize) {
        self.adjacent_node_indices.push(index);
    }
}
