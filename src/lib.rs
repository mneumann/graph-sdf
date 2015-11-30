extern crate petgraph;

use std::str::FromStr;
use std::fmt::Debug;
use std::io::BufRead;

use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph};

pub trait Visitor<NodeWt, EdgeWt> {
    fn init(&mut self, num_nodes: usize, num_edges: usize);
    fn node(&mut self, node_id: usize, node_weight: Option<NodeWt>);
    fn endnode(&mut self, _node_id: usize) {
    }
    fn edge(&mut self, src_id: usize, dst_id: usize, edge_weight: Option<EdgeWt>);
}

pub fn read_sgf<R, V, NodeWt, EdgeWt, I>(rd: &mut R, visitor: &mut V)
    where R: BufRead,
          V: Visitor<NodeWt, EdgeWt>,
          NodeWt: FromStr<Err = I>,
          EdgeWt: FromStr<Err = I>,
          I: Debug
{
    let mut meta: Option<(usize, usize)> = None;

    for line in rd.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.starts_with("#") {
            // skip comment
            continue;
        }

        if meta.is_none() {
            // First non-comment line is expected to be meta
            let mut m = line.split_whitespace();
            let _version = match m.next() {
                Some("SGF/1") => {
                    1
                }
                _ => {
                    panic!("Invalid format");
                }
            };
            let num_nodes: usize = match m.next() {
                Some(ns) => {
                    ns.parse().unwrap()
                }
                _ => {
                    panic!("Invalid format");
                }
            };
            let num_edges: usize = match m.next() {
                Some(ns) => {
                    ns.parse().unwrap()
                }
                _ => {
                    panic!("Invalid format");
                }
            };

            meta = Some((num_nodes, num_edges));

            visitor.init(num_nodes, num_edges);
        } else {
            let mut i = line.splitn(2, '|');
            let (node_id, node_weight) = match i.next() {
                Some(ns) => {
                    let mut it = ns.splitn(2, ":");
                    let node_id: usize = it.next().unwrap().parse().unwrap();
                    let node_weight: Option<NodeWt> = it.next().map(|s| s.parse().unwrap());
                    (node_id, node_weight)
                }
                _ => {
                    panic!("Invalid format");
                }
            };
            visitor.node(node_id, node_weight);

            let edge_s = i.next().unwrap();
            for es in edge_s.split(',') {
                let mut it = es.splitn(2, ":");
                let target_id: usize = it.next()
                                         .unwrap()
                                         .parse()
                                         .unwrap();

                let edge_weight: Option<EdgeWt> = it.next()
                                                    .map(|s| s.parse::<EdgeWt>().unwrap());

                visitor.edge(node_id, target_id, edge_weight);
            }

            visitor.endnode(node_id);
        }
    }
}

pub struct PetgraphReader<NodeWt: Default, EdgeWt: Default> {
    graph: Graph<NodeWt, EdgeWt, Directed>,
}

impl<NodeWt, EdgeWt, I> PetgraphReader<NodeWt, EdgeWt>
where NodeWt: Default+FromStr<Err = I>,
      EdgeWt: Default+FromStr<Err = I>,
      I:Debug
{
    pub fn new() -> PetgraphReader<NodeWt, EdgeWt> {
        PetgraphReader { graph: Graph::new() }
    }

    pub fn from_sgf<R: BufRead>(rd: &mut R) -> Graph<NodeWt, EdgeWt, Directed> {
        let mut visitor = Self::new();
        read_sgf(rd, &mut visitor);
        return visitor.graph;
    }
}

impl<NodeWt:Default, EdgeWt:Default> Visitor<NodeWt, EdgeWt> for PetgraphReader<NodeWt, EdgeWt> {
    fn init(&mut self, num_nodes: usize, _num_edges: usize) {
        for _ in 0..num_nodes {
            self.graph.add_node(NodeWt::default());
        }
    }

    fn node(&mut self, node_id: usize, node_weight: Option<NodeWt>) {
        *self.graph.node_weight_mut(NodeIndex::new(node_id)).unwrap() =
            node_weight.unwrap_or(NodeWt::default());
    }

    fn edge(&mut self, src_id: usize, dst_id: usize, edge_weight: Option<EdgeWt>) {
        let _ = self.graph.add_edge(NodeIndex::new(src_id),
                                    NodeIndex::new(dst_id),
                                    edge_weight.unwrap_or(EdgeWt::default()));
    }
}
