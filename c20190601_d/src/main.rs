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

        if len > 0 {
            let len = len - 1;
            if self.bytes_buffer[len] == delimiter {
                self.bytes_buffer.truncate(len);
            }
        }

        &self.bytes_buffer
    }

    fn scan_u(&mut self, delimiter: u8) -> u32 {
        self.read(delimiter)
            .into_iter()
            .fold(0, |accumulator, byte| {
                (byte - b'0') as u32 + accumulator * 10
            })
    }

    fn scan_u_vec(&mut self, n: usize, separator: u8, delimiter: u8) -> Vec<u32> {
        let mut vector = Vec::with_capacity(n);
        for _i in 0..n - 1 {
            vector.push(self.scan_u(separator));
        }
        vector.push(self.scan_u(delimiter));
        vector
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
        for point in iterator.by_ref().take(length - 1) {
            self.add(point, separator);
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

fn create_populated_vec<T, F>(capacity: usize, factory: F) -> Vec<T>
where
    F: Fn(usize) -> T,
{
    let mut vector: Vec<T> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        vector.push(factory(i));
    }
    vector
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

struct Node {
    adjacent_node_indices: Vec<usize>,
    point: u32,
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
    let cin = &mut create_cin(cin.lock(), 8);

    // ---- input ----
    let n = cin.scan_u(LF) as usize;

    let mut nodes: Vec<Node> = create_populated_vec(n, |_| Node {
        adjacent_node_indices: Vec::with_capacity(n / 4),
        point: 0,
    });

    scan_adjacent_nodes(cin, n - 1, &mut nodes);

    let mut c = cin.scan_u_vec(n, SP, LF);

    // ---- operation ----
    c.sort(); // ascending, so that c.pop() will be the max
    let max_c = c[n - 1];
    let mut score: u32 = 0;

    {
        let mut visit_callback = |node: &mut Node| {
            let point = c.pop().unwrap();
            node.point = point;
            score += point;
            false
        };

        depth_first_search(&mut nodes, n, 0, &mut visit_callback);
    }

    score -= max_c;

    // ---- output ----
    let cout = &mut create_cout(16 + (max_c.to_string().len() + 1) * n);
    cout.add(score, '\n');

    let mut node_points = nodes.iter().map(|node| node.point);
    cout.add_from_iterator(&mut node_points, n, ' ', '\n');

    cout.flush();
}
