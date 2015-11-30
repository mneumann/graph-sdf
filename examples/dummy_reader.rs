extern crate graph_sgf;
use graph_sgf::{Visitor, read_sgf};
use std::io::BufReader;
use std::fs::File;

struct DummyReader;

impl Visitor<f32, f32> for DummyReader {
    fn init(&mut self, num_nodes: usize, num_edges: usize) {
        println!("num_nodes: {}, num_edges: {}", num_nodes, num_edges);
    }
    fn node(&mut self, node_id: usize, node_weight: Option<f32>) {
        println!("node id: {} weight: {:?}", node_id, node_weight);
    }
    fn edge(&mut self, src_id: usize, dst_id: usize, edge_weight: Option<f32>) {
        println!("edge from {} to {} with weight: {:?}",
                 src_id,
                 dst_id,
                 edge_weight);
    }
}

fn main() {
    let f = File::open("er_100_0_1.sgf").unwrap();
    let mut f = BufReader::new(f);
    let mut dummy_reader = DummyReader;
    read_sgf(&mut f, &mut dummy_reader);
}
