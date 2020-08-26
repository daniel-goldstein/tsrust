use std::cmp;
use std::cmp::Ordering;
use std::vec::Vec;

type NodeId = std::option::Option<usize>;

const EPS: f64 = 1e-6;

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
        Edge { parent, child, left, right }
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
        TreeSequence { edges: Vec::new() }
    }

    fn add_edge(&mut self, e: Edge) {
        match self.edges.binary_search(&e) {
            Ok(pos) => self.edges.insert(pos, e),
            Err(pos) => self.edges.insert(pos, e),
        }
    }
}

// TODO: Support list of children -> parent and left/right
#[macro_export]
macro_rules! treeseq {
    ( $( $u:literal -> $v:literal ),* ) => {
        {
            let mut ts = TreeSequence::new();
            $(
                ts.add_edge(Edge::new($v, $u, 0, 1));
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
    use super::{Tree, TreeSequence, Edge};

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
        ts.add_edge(Edge::new(0, 1, 0, 1));
        ts.add_edge(Edge::new(0, 2, 0, 1));

        assert_eq!(treeseq!(1 -> 0, 2 -> 0), ts);
    }
}
