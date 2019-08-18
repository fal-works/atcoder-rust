use std::io::*;

const SP: u8 = b' ';
const LF: u8 = b'\n';

struct CharacterInput<'a> {
    locked_stdin: StdinLock<'a>,
    bytes_buffer: Vec<u8>,
}

impl<'a> CharacterInput<'a> {
    fn read(&mut self, delimiter: u8) -> &[u8] {
        self.bytes_buffer.clear();

        let len = self
            .locked_stdin
            .read_until(delimiter, &mut self.bytes_buffer)
            .unwrap();

        let end_index = match self.bytes_buffer.last() {
            Some(byte) => {
                if *byte == delimiter {
                    len - 1
                } else {
                    len
                }
            }
            None => len,
        };

        &self.bytes_buffer[0..end_index]
    }

    fn scan_u(&mut self, delimiter: u8) -> u32 {
        self.read(delimiter)
            .into_iter()
            .fold(0, |accumulator, byte| {
                (byte - b'0') as u32 + accumulator * 10
            })
    }
}

fn create_cin<'a>(input: StdinLock<'a>, buffer_capacity: usize) -> CharacterInput {
    CharacterInput {
        locked_stdin: input,
        bytes_buffer: Vec::with_capacity(buffer_capacity),
    }
}

struct CharacterOutput {
    string_buffer: String,
}

impl CharacterOutput {
    fn push_s(&mut self, s: &str) {
        self.string_buffer += s;
    }

    fn push<T: ToString>(&mut self, n: T) {
        self.push_s(&n.to_string());
    }

    fn add<T: ToString>(&mut self, n: T, separator: char) {
        self.push(n);
        self.string_buffer.push(separator);
    }

    fn add_from_iterator<T: ToString, I>(
        &mut self,
        iterator: &mut I,
        length: usize,
        separator: char,
        delimiter: char,
    ) where
        I: Iterator<Item = T>,
    {
        for element in iterator.by_ref().take(length - 1) {
            self.add(element, separator);
        }
        self.add(iterator.next().unwrap(), delimiter);
    }

    fn flush(&mut self) {
        let out = stdout();
        let mut out = out.lock();
        out.write_all(self.string_buffer.as_bytes()).unwrap();
        self.string_buffer.clear();
    }
}

fn create_cout(capacity: usize) -> CharacterOutput {
    CharacterOutput {
        string_buffer: String::with_capacity(capacity),
    }
}

// ------------------------------------------------------------------------

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

struct Node {
    adjacent_node_indices: Vec<usize>,
}

impl Node {
    // fn new_list(length: usize) -> Vec<Node> {
    //     (0..length)
    //         .map(|_| Node {
    //             adjacent_node_indices: Vec::new(),
    //         })
    //         .collect()
    // }

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

// ------------------------------------------------------------------------

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 16);

    let n = cin.scan_u(SP) as usize;
    let q = cin.scan_u(LF) as usize;

    let mut node_list = Node::new_list_with_capacity(n, 2);
    scan_adjacent_nodes(cin, n - 1, &mut node_list);

    let mut count_list: Vec<u32> = vec![0; n];

    for _ in 0..q {
        let p = cin.scan_u(SP) as usize;
        let x = cin.scan_u(LF);
        count_list[p - 1] += x;
    }

    {
        let mut visit_callback = |node_index: usize, parent_node_index: Option<usize>| {
            let parent_total_count = parent_node_index.map(|i| count_list[i]).unwrap_or(0);
            count_list[node_index] += parent_total_count;
            return false;
        };

        breadth_first_search(&mut node_list, n, 0, &mut visit_callback);
    }

    let cout = &mut create_cout(8 * n);
    cout.add_from_iterator(&mut count_list.iter(), n, ' ', '\n');
    cout.flush();
}
