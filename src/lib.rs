use std::cmp;
use std::cmp::Ordering;
use std::vec::Vec;

type NodeId = std::option::Option<usize>;

#[derive(PartialEq, Eq, Debug)]
pub struct Edge {
    parent: usize,
    child: usize,
    left: u64,
    right: u64,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.left, self.parent, self.child).cmp(&(other.left, other.parent, other.child))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Edge {
    fn new(parent: usize, child: usize, left: u64, right: u64) -> Edge {
        Edge {
            parent,
            child,
            left,
            right,
        }
    }
}

// Currently just represents whether the given node is a sample.
// If false, the Node is not a sample and is instead an inferred ancestor.
type Node = bool;

#[derive(PartialEq, Eq, Debug)]
pub struct TreeSequence {
    edges: Vec<Edge>,
}

pub struct Tree {
    parent: Vec<NodeId>,
}

impl TreeSequence {
    fn new() -> Self {
        TreeSequence { edges: vec![] }
    }

    fn add_edge(&mut self, child: usize, parent: usize, left: u64, right: u64) {
        let e = Edge::new(child, parent, left, right);
        match self.edges.binary_search(&e) {
            Ok(pos) => self.edges.insert(pos, e),
            Err(pos) => self.edges.insert(pos, e),
        }
    }
}

// TODO Consider how this works for things that start as roots
pub struct TreeSequenceBuilder {
    ts: TreeSequence,
    last_breakpoint: u64,
    curr_edges: Vec<(usize, usize, u64)>, // (child, parent, left)
}

impl TreeSequenceBuilder {
    fn new() -> Self {
        TreeSequenceBuilder {
            ts: TreeSequence::new(),
            last_breakpoint: 0,
            curr_edges: vec![],
        }
    }

    fn insert(mut self, children: Vec<usize>, parent: usize) -> Self {
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

    fn transplant(mut self, children: Vec<usize>, new_parent: Option<usize>) -> Self {
        for c in children {
            match self.curr_edges.iter().position(|(child, _, _)| *child == c) {
                Some(index) => {
                    let (child, old_parent, left) = self.curr_edges[index];
                    self.curr_edges.remove(index);
                    // The tracked edge now ends at last_breakpoint and we start
                    // a new ongoing edge from there
                    self.ts
                        .add_edge(child, old_parent, left, self.last_breakpoint);
                    if let Some(new_parent) = new_parent {
                        self.curr_edges
                            .push((child, new_parent, self.last_breakpoint));
                    }
                }
                None => panic!("Can't move child node that does not yet exist"),
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

impl Tree {
    fn new(parent: Vec<NodeId>) -> Self {
        Tree { parent }
    }

    fn parent(&self, u: usize) -> NodeId {
        self.parent.get(u)?.clone()
    }

    fn mrca(&self, u: usize, v: usize) -> NodeId {
        let u_anc = self.ancestor_chain(u);
        let v_anc = self.ancestor_chain(v);

        if u_anc.last().unwrap() != v_anc.last().unwrap() {
            return None;
        }
        let mut common_ancestor = u_anc.last().unwrap().clone();

        let mut i = 0;
        let u_len = u_anc.len();
        let v_len = v_anc.len();
        while i < cmp::min(u_len, v_len) {
            if u_anc[u_len - i - 1] != v_anc[v_len - i - 1] {
                return Some(common_ancestor);
            }
            common_ancestor = u_anc[u_len - i - 1];
            i += 1;
        }
        Some(common_ancestor)
    }

    // Guaranteed to return a non-empty Vec
    fn ancestor_chain(&self, x: usize) -> Vec<usize> {
        let mut chain = vec![x];
        let mut child = x;
        while let Some(parent) = self.parent(child) {
            chain.push(parent);
            child = parent;
        }
        chain
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parent() {
        let t = Tree::new(vec![None, Some(0), Some(0)]);
        assert_eq!(t.parent(0), None);
        assert_eq!(t.parent(1), Some(0));
        assert_eq!(t.parent(2), Some(0));
    }

    #[test]
    fn test_mrca() {
        let t = Tree::new(vec![None, Some(0), Some(0)]);
        assert_eq!(t.mrca(0, 1), Some(0));
        assert_eq!(t.mrca(0, 2), Some(0));
        assert_eq!(t.mrca(1, 2), Some(0));

        let t2 = Tree::new(vec![None, Some(0), Some(1), Some(2)]);
        assert_eq!(t2.mrca(0, 1), Some(0));
        assert_eq!(t2.mrca(0, 2), Some(0));
        assert_eq!(t2.mrca(0, 3), Some(0));
        assert_eq!(t2.mrca(1, 0), Some(0));
        assert_eq!(t2.mrca(1, 2), Some(1));
        assert_eq!(t2.mrca(1, 3), Some(1));
        assert_eq!(t2.mrca(2, 0), Some(0));
        assert_eq!(t2.mrca(2, 1), Some(1));
        assert_eq!(t2.mrca(2, 3), Some(2));
        assert_eq!(t2.mrca(3, 0), Some(0));
        assert_eq!(t2.mrca(3, 1), Some(1));
        assert_eq!(t2.mrca(3, 2), Some(2));
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
