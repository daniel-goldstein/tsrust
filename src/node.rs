pub type NodeId = usize;

// TODO Track node list
// Currently just represents whether the given node is a sample.
// If false, the Node is not a sample and is instead an inferred ancestor.
type Node = bool;
