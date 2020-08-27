use crate::node::NodeId;
use std::cmp;

#[derive(PartialEq, Eq, Debug)]
pub struct Tree {
    pub parent: Vec<Option<NodeId>>,
}

impl Tree {
    pub fn new(parent: Vec<Option<NodeId>>) -> Self {
        Tree { parent }
    }

    pub fn parent(&self, u: NodeId) -> Option<NodeId> {
        self.parent.get(u)?.clone()
    }

    /// Returns the most recent common ancestor of two nodes in the `Tree`.
    /// `None` is returned if the nodes do not share a common ancestor
    /// (they are under different roots).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tsrust::tree::Tree;
    ///
    /// let t = Tree::new(vec![None, Some(0), Some(0)]);
    /// assert_eq!(t.mrca(1, 2), Some(0));
    ///
    /// let t = Tree::new(vec![None, None]);
    /// assert_eq!(t.mrca(0, 1), None);
    /// ```
    pub fn mrca(&self, u: NodeId, v: NodeId) -> Option<NodeId> {
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
    fn ancestor_chain(&self, x: NodeId) -> Vec<NodeId> {
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
    use super::Tree;

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
}
