use tsrust::treeseq::*;

fn main() {
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

    for (tree_index, t) in ts.iter().enumerate() {
        println!("Tree {}: {:?}", tree_index, t.parent);
    }
}
