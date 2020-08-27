use crate::node::NodeId;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Edge {
    pub parent: NodeId,
    pub child: NodeId,
    pub left: u64,
    pub right: u64,
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
