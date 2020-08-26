use crate::node::NodeId;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub struct Edge {
    parent: NodeId,
    child: NodeId,
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
    pub fn new(parent: NodeId, child: NodeId, left: u64, right: u64) -> Edge {
        Edge {
            parent,
            child,
            left,
            right,
        }
    }
}
