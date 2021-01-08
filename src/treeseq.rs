use crate::edge::Edge;
use crate::node::NodeId;
use crate::tree::Tree;
use std::cmp;
use std::vec::Vec;

use streaming_iterator::StreamingIterator;

#[derive(PartialEq, Eq, Debug)]
pub struct TreeSequence {
    num_nodes: usize,
    edges: Vec<Edge>,
}

impl TreeSequence {
    pub fn new() -> Self {
        TreeSequence {
            num_nodes: 0,
            edges: vec![],
        }
    }

    pub fn iter(&self) -> TreeSequenceIterator {
        let edges: Vec<&Edge> = self.edges.iter().rev().collect();
        TreeSequenceIterator::new(self.num_nodes, edges)
    }

    pub fn streaming_iter(&self) -> TreeSequenceStreamingIterator {
        let edges: Vec<&Edge> = self.edges.iter().rev().collect();
        TreeSequenceStreamingIterator::new(self.num_nodes, edges)
    }

    pub fn for_each_with_index<F>(&self, f: F)
    where
        F: Fn(&Tree, usize),
    {
        let mut iter = self.streaming_iter();
        let mut tree_index = 0;
        while let Some(t) = iter.next() {
            f(t, tree_index);
            tree_index += 1;
        }
    }

    pub fn add_edge(&mut self, child: NodeId, parent: NodeId, left: u64, right: u64) {
        let e = Edge {
            child,
            parent,
            left,
            right,
        };
        self.num_nodes = cmp::max(self.num_nodes, cmp::max(child, parent) + 1);
        match self.edges.binary_search(&e) {
            Ok(_) => panic!("Cannot have duplicate edges"),
            Err(pos) => self.edges.insert(pos, e),
        }
    }
}

pub struct TreeSequenceIterator<'a> {
    num_nodes: usize,
    current_edges: Vec<&'a Edge>,
    upcoming_edges: Vec<&'a Edge>,
}

impl<'a> TreeSequenceIterator<'a> {
    fn new(num_nodes: usize, edges: Vec<&'a Edge>) -> Self {
        TreeSequenceIterator {
            num_nodes,
            current_edges: vec![],
            upcoming_edges: edges,
        }
    }
}

impl Iterator for TreeSequenceIterator<'_> {
    type Item = Tree;

    // TODO way to index edges by right index so we can quickly remove them.
    // outgoing edges. upcoming_edges are already sorted by left endpoint so we
    // don't have this problem for figuring out how many new edges to pull in.
    fn next(&mut self) -> Option<Self::Item> {
        // Remove outgoing edges
        if let Some(&out_edge) = self.current_edges.iter().min_by_key(|&&e| e.right) {
            self.current_edges.retain(|&e| e.right > out_edge.right);
        }

        // Add incoming edges
        if let Some(&e) = self.upcoming_edges.last() {
            let new_right = e.left;
            while let Some(&e) = self.upcoming_edges.last() {
                if e.left > new_right {
                    break;
                }
                self.current_edges.push(self.upcoming_edges.pop().unwrap());
            }
        }

        if self.current_edges.is_empty() && self.upcoming_edges.is_empty() {
            None
        } else {
            // This is interesting. Ideally I would want to mantain a mut Tree
            // in the iterator and return a reference to it. It's significantly
            // faster to update the children of outgoing and incoming edges each
            // time than to construct the whole tree again...
            // It doesn't look this is a very easy thing to do without
            // generic associative types...
            // See StreamingIterator below for the "solution" to this
            let mut parent: Vec<Option<NodeId>> = vec![];
            parent.resize_with(self.num_nodes, Default::default);
            for &e in &self.current_edges {
                parent[e.child] = Some(e.parent);
            }

            Some(Tree::new(parent))
        }
    }
}

pub struct TreeSequenceStreamingIterator<'a> {
    tree: Tree,
    current_edges: Vec<&'a Edge>,
    upcoming_edges: Vec<&'a Edge>,
}

impl<'a> TreeSequenceStreamingIterator<'a> {
    fn new(num_nodes: usize, edges: Vec<&'a Edge>) -> Self {
        let mut parent: Vec<Option<NodeId>> = vec![];
        parent.resize_with(num_nodes, Default::default);
        TreeSequenceStreamingIterator {
            tree: Tree::new(parent),
            current_edges: vec![],
            upcoming_edges: edges,
        }
    }
}

impl StreamingIterator for TreeSequenceStreamingIterator<'_> {
    type Item = Tree;

    fn advance(&mut self) {
        // Remove outgoing edges
        if let Some(&out_edge) = self.current_edges.iter().min_by_key(|&&e| e.right) {
            for e in self.current_edges.iter() {
                if e.right == out_edge.right {
                    self.tree.set_parent(e.child, None);
                }
            }
            self.current_edges.retain(|&e| e.right > out_edge.right);
        }

        // Add incoming edges
        if let Some(&e) = self.upcoming_edges.last() {
            let new_right = e.left;
            while let Some(&e) = self.upcoming_edges.last() {
                if e.left > new_right {
                    break;
                }
                let new_edge = self.upcoming_edges.pop().unwrap();
                self.current_edges.push(new_edge);
                self.tree.set_parent(e.child, Some(e.parent));
            }
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.current_edges.is_empty() && self.upcoming_edges.is_empty() {
            None
        } else {
            Some(&self.tree)
        }
    }
}
