use crate::edge::Edge;
use crate::node::NodeId;
use crate::tree::Tree;
use std::cmp;
use std::vec::Vec;

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

    fn add_edge(&mut self, child: NodeId, parent: NodeId, left: u64, right: u64) {
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

    // TODO current_edges should be sorted by right endpoint so it's quick to remove
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
            let mut parent: Vec<Option<NodeId>> = vec![];
            parent.resize_with(self.num_nodes, || None);
            for &e in &self.current_edges {
                parent[e.child] = Some(e.parent);
            }

            Some(Tree::new(parent))
        }
    }
}

pub struct TreeSequenceBuilder {
    ts: TreeSequence,
    last_breakpoint: u64,
    curr_edges: Vec<(NodeId, NodeId, u64)>, // (child, parent, left)
}

impl TreeSequenceBuilder {
    pub fn new() -> Self {
        TreeSequenceBuilder {
            ts: TreeSequence::new(),
            last_breakpoint: 0,
            curr_edges: vec![],
        }
    }

    pub fn insert(mut self, children: Vec<NodeId>, parent: NodeId) -> Self {
        for c in children {
            self.curr_edges
                .push((c, parent, self.last_breakpoint));
        }
        self
    }

    pub fn breakpoint(mut self, breakpoint: u64) -> Self {
        self.last_breakpoint = breakpoint;
        self
    }

    pub fn transplant(mut self, children: Vec<NodeId>, new_parent: Option<NodeId>) -> Self {
        for c in children {
            // Flush the existing edge for that child if there is one
            if let Some(index) = self.curr_edges.iter().position(|(child, _, _)| *child == c) {
                let (child, old_parent, left) = self.curr_edges.remove(index);
                self.ts
                    .add_edge(child, old_parent, left, self.last_breakpoint);
            }
            // Start a new edge for the child if it has a new parent
            if let Some(new_parent) = new_parent {
                self.curr_edges.push((c, new_parent, self.last_breakpoint));
            }
        }

        self
    }

    pub fn end(mut self, seq_length: u64) -> TreeSequence {
        for (child, parent, left) in self.curr_edges {
            self.ts.add_edge(child, parent, left, seq_length);
        }

        self.ts
    }
}

// Trying to see if I can use a macro instead of the builder for fun
// TODO: Support list of children -> parent and left/right
#[macro_export]
macro_rules! treeseq {
    ( $( $u:literal -> $v:literal ),* ) => {
        {
            let mut ts = TreeSequence::new();
            $(
                ts.add_edge($u, $v, 0, 1);
            )*
            ts
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_ts() -> TreeSequence {
        TreeSequenceBuilder::new()
            .insert(vec![0, 1], 4)
            .insert(vec![2, 3], 5)
            .insert(vec![4, 5], 6)
            .breakpoint(1)
            .transplant(vec![0], Some(6))
            .transplant(vec![1], Some(5))
            .breakpoint(2)
            .transplant(vec![0, 5], None)
            .end(3)
    }

    #[test]
    fn test_tree_sequence_builder() {
        let mut hand_rolled = TreeSequence::new();
        hand_rolled.add_edge(0, 4, 0, 1);
        hand_rolled.add_edge(1, 4, 0, 1);
        hand_rolled.add_edge(2, 5, 0, 3);
        hand_rolled.add_edge(3, 5, 0, 3);
        hand_rolled.add_edge(4, 6, 0, 3);
        hand_rolled.add_edge(5, 6, 0, 2);

        hand_rolled.add_edge(0, 6, 1, 2);
        hand_rolled.add_edge(1, 5, 1, 3);

        assert_eq!(example_ts(), hand_rolled);
    }

    #[test]
    fn test_tree_sequence_builder_empty() {
        assert_eq!(TreeSequence::new(), TreeSequenceBuilder::new().end(1));
    }

    #[test]
    fn test_tree_sequence_macro() {
        let mut ts = TreeSequence::new();
        ts.add_edge(1, 0, 0, 1);
        ts.add_edge(2, 0, 0, 1);
        assert_eq!(treeseq!(1 -> 0, 2 -> 0), ts);

        let ts = TreeSequenceBuilder::new().insert(vec![1, 2], 0).end(1);
        assert_eq!(treeseq!(1 -> 0, 2 -> 0), ts);
    }

    #[test]
    fn test_tree_sequence_iter() {
        let ts = example_ts();
        let mut ts_iter = ts.iter();

        let t1 = Tree::new(vec![
            Some(4),
            Some(4),
            Some(5),
            Some(5),
            Some(6),
            Some(6),
            None,
        ]);
        assert_eq!(ts_iter.next(), Some(t1));

        let t2 = Tree::new(vec![
            Some(6),
            Some(5),
            Some(5),
            Some(5),
            Some(6),
            Some(6),
            None,
        ]);
        assert_eq!(ts_iter.next(), Some(t2));

        let t2 = Tree::new(vec![None, Some(5), Some(5), Some(5), Some(6), None, None]);
        assert_eq!(ts_iter.next(), Some(t2));

        assert_eq!(ts_iter.next(), None);
    }
}
