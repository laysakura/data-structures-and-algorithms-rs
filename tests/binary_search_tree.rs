use data_structures_and_algorithms_rs::BinarySearchTree;

#[test]
fn leetcode_938_range_sum_of_bst() {
    //          10
    //         /  \
    //        5    15
    //       / \     \
    //      3   7     18
    let mut bst1 = BinarySearchTree::new();
    bst1.add(10);
    bst1.add(5);
    bst1.add(3);
    bst1.add(7);
    bst1.add(15);
    bst1.add(18);
    assert_eq!(bst1.get_range_sorted(&7, &15).into_iter().sum::<i32>(), 32);

    //          10
    //       __/  \__
    //      /        \
    //     5         15
    //    / \       /  \
    //   3   7     13   18
    //  /   /
    // 1   6
    let mut bst2 = BinarySearchTree::new();
    bst2.add(10);
    bst2.add(5);
    bst2.add(3);
    bst2.add(7);
    bst2.add(1);
    bst2.add(6);
    bst2.add(15);
    bst2.add(13);
    bst2.add(18);
    assert_eq!(bst2.get_range_sorted(&6, &10).into_iter().sum::<i32>(), 23);
}
