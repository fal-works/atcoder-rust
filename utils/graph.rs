
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
    visit_callback: &mut F,
) -> Option<usize>
where
    T: GraphNode,
    F: FnMut(&mut T) -> bool,
{
    let mut node_visited_indicator_table = vec![false; n];

    let mut frontier_index_stack: Vec<usize> = Vec::with_capacity(n / 2);
    frontier_index_stack.push(start_index);

    loop {
        let node_index = frontier_index_stack.pop();

        match node_index {
            Some(current) => {
                let mut node = &mut nodes[current];
                if visit_callback(node) {
                    return Some(current);
                }

                for neighbor in node
                    .get_adjacent_node_indices()
                    .iter()
                    .map(|rf| *rf)
                    .filter(|neighbor| node_visited_indicator_table[*neighbor] == false)
                {
                    frontier_index_stack.push(neighbor); // for_each() is not supported by v1.15.1
                }

                node_visited_indicator_table[current] = true;
            }
            None => break,
        }
    }

    None
}

// ------------------------------------------------------------------------

struct NodeSample {
    adjacent_node_indices: Vec<usize>,
}

impl GraphNode for Node {
    fn get_adjacent_node_indices(&self) -> &Vec<usize> {
        &self.adjacent_node_indices
    }

    fn add_adjacent_node(&mut self, index: usize) {
        self.adjacent_node_indices.push(index);
    }
}
