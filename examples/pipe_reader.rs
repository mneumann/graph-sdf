extern crate graph_sgf;
use graph_sgf::{Visitor, read_sgf};
use std::io::BufReader;
use std::fs::File;

// Pipes input to output, removing only comments
struct PipeReader {
    first_edge: bool,
}

impl PipeReader {
    fn new() -> PipeReader {
        PipeReader { first_edge: true }
    }
}

impl Visitor<f32, f32> for PipeReader {
    fn init(&mut self, num_nodes: usize, num_edges: usize) {
        println!("SGF/1 {} {}", num_nodes, num_edges);
    }
    fn node(&mut self, node_id: usize, node_weight: Option<f32>) {
        print!("{}", node_id);
        if let Some(w) = node_weight {
            print!("{:?}", w);
        }
        print!("|");
        self.first_edge = true;
    }
    fn endnode(&mut self, _node_id: usize) {
        println!("");
    }

    fn edge(&mut self, _src_id: usize, dst_id: usize, edge_weight: Option<f32>) {
        if self.first_edge {
            self.first_edge = false;
        } else {
            print!(",");
        }
        print!("{}", dst_id);

        if let Some(w) = edge_weight {
            print!("{:?}", w);
        }
    }
}

fn main() {
    let f = File::open("path_30.sgf").unwrap();
    let mut f = BufReader::new(f);
    let mut pipe_reader = PipeReader::new();
    read_sgf(&mut f, &mut pipe_reader);
}
