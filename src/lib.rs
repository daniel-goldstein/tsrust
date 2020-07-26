use std::vec::Vec;

type NodeId = std::option::Option<usize>;

pub struct Edge {
    parent: NodeId,
    child: NodeId,
    left: f64,
    right: f64,
}

pub struct TreeSequence {
    edges: Vec<Edge>,
}

pub struct Tree {
    parent: Vec<NodeId>,
    // left_child: Vec<NodeId>,
    // right_child: Vec<NodeId>,
}

impl TreeSequence {
    fn new() -> Self {
        TreeSequence { edges: Vec::new() }
    }

    fn add_edge(&self) {}
}

impl Tree {
    fn new(parent: Vec<NodeId>) -> Self {
        Tree { parent }
    }

    fn parent(&self, u: usize) -> NodeId {
        self.parent.get(u)?.clone()
    }

    fn mrca(&self, u: usize, v: usize) -> NodeId {
        let mut u_path = Vec::new();
        let mut v_path = Vec::new();

        let p = u;
        while let Some(p) = self.parent(p) {
            u_path.push(p);
        }

        let p = v;
        while let Some(p) = self.parent(p) {
            v_path.push(p);
        }

        let mrca = u_path
            .iter()
            .zip(v_path.iter())
            .find(|(ref u1, ref v1)| u1 == v1)?
            .0;
        Some(mrca.clone())
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

    fn test_mrca() {
        let t = Tree::new(vec![None, Some(0), Some(0)]);
        assert_eq!(t.mrca(1, 2), Some(0));
        assert_eq!(t.mrca(0, 2), Some(0));
        assert_eq!(t.mrca(0, 1), Some(0));
    }
}
