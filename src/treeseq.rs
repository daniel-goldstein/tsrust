use std::vec::Vec;
use crate::edge::Edge;
use crate::node::NodeId;

#[derive(PartialEq, Eq, Debug)]
pub struct TreeSequence {
    edges: Vec<Edge>,
}

impl TreeSequence {
    fn new() -> Self {
        TreeSequence { edges: vec![] }
    }

    fn add_edge(&mut self, child: NodeId, parent: NodeId, left: u64, right: u64) {
        let e = Edge::new(child, parent, left, right);
        match self.edges.binary_search(&e) {
            Ok(pos) => self.edges.insert(pos, e),
            Err(pos) => self.edges.insert(pos, e),
        }
    }
}

pub struct TreeSequenceBuilder {
    ts: TreeSequence,
    last_breakpoint: u64,
    curr_edges: Vec<(NodeId, NodeId, u64)>, // (child, parent, left)
}

impl TreeSequenceBuilder {
    fn new() -> Self {
        TreeSequenceBuilder {
            ts: TreeSequence::new(),
            last_breakpoint: 0,
            curr_edges: vec![],
        }
    }

    fn insert(mut self, children: Vec<NodeId>, parent: NodeId) -> Self {
        for c in children {
            self.curr_edges
                .push((c, parent.clone(), self.last_breakpoint));
        }
        self
    }

    fn breakpoint(mut self, breakpoint: u64) -> Self {
        self.last_breakpoint = breakpoint;
        self
    }

    fn transplant(mut self, children: Vec<NodeId>, new_parent: Option<NodeId>) -> Self {
        for c in children {
            // Flush the existing edge for that child if there is one
            if let Some(index) = self.curr_edges.iter().position(|(child, _, _)| *child == c) {
                let (child, old_parent, left) = self.curr_edges.remove(index);
                self.ts
                    .add_edge(child, old_parent, left, self.last_breakpoint);
            }
            // Start a new edge for the child it is given to a new parent
            if let Some(new_parent) = new_parent {
                self.curr_edges.push((c, new_parent, self.last_breakpoint));
            }
        }

        self
    }

    fn end(mut self, seq_length: u64) -> TreeSequence {
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
    fn test_tree_sequence_builder() {
        let ts = TreeSequenceBuilder::new()
            .insert(vec![0, 1], 4)
            .insert(vec![2, 3], 5)
            .insert(vec![4, 5], 6)
            .breakpoint(1)
            .transplant(vec![0], Some(6))
            .transplant(vec![1], Some(5))
            .breakpoint(2)
            .transplant(vec![0, 5], None)
            .end(3);

        let mut hand_rolled = TreeSequence::new();
        hand_rolled.add_edge(0, 4, 0, 1);
        hand_rolled.add_edge(1, 4, 0, 1);
        hand_rolled.add_edge(2, 5, 0, 3);
        hand_rolled.add_edge(3, 5, 0, 3);
        hand_rolled.add_edge(4, 6, 0, 3);
        hand_rolled.add_edge(5, 6, 0, 2);

        hand_rolled.add_edge(0, 6, 1, 2);
        hand_rolled.add_edge(1, 5, 1, 3);

        assert_eq!(ts, hand_rolled);
    }

    #[test]
    fn test_tree_sequence_builder_empty() {
        assert_eq!(TreeSequence::new(), TreeSequenceBuilder::new().end(1));
    }
}