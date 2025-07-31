use flameview::{FlameTree};

#[test]
fn insert_preserves_counts() {
    let mut tree = FlameTree::new();
    let root = tree.root();
    let a = tree.insert_child(root, "a", 2);
    assert_eq!(tree.total_samples(), 2);
    let b = tree.insert_child(root, "b", 3);
    assert_eq!(tree.total_samples(), 5);
    let c = tree.insert_child(a, "c", 5);
    assert_eq!(tree.total_samples(), 10);
    assert_eq!(tree[a].total_count, 7);
    assert_eq!(tree[b].total_count, 3);
    assert_eq!(tree[c].total_count, 5);
    assert_eq!(tree[root].first_child, Some(b));
    assert_eq!(tree[b].next_sibling, Some(a));
    assert_eq!(tree[a].first_child, Some(c));
    assert_eq!(tree[c].next_sibling, None);
}
