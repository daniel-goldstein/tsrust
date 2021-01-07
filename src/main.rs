use tsrust::treeseqbuilder::TreeSequenceBuilder;
use streaming_iterator::StreamingIterator;

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

    // With iterator that generates a new Tree every time
    for (tree_index, t) in ts.iter().enumerate() {
        for node in t.nodes() {
            println!(
                "Tree {}: {} has parent {:?}",
                tree_index,
                node,
                t.parent(node)
            );
        }
    }

    // With only 1 Tree allocation for the whole iteration
    let mut ts_iter = ts.streaming_iter();
    let mut tree_index = 0;
    while let Some(t) = ts_iter.next() {
        for node in t.nodes() {
            println!(
                "Tree {}: {} has parent {:?}",
                tree_index,
                node,
                t.parent(node)
            );
        }
        tree_index += 1;
    }

    // Hiding the ugly parts
    ts.for_each_with_index(|t, tree_index| {
        for node in t.nodes() {
            println!(
                "Tree {}: {} has parent {:?}",
                tree_index,
                node,
                t.parent(node)
            );
        }
    });
}
