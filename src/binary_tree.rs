#[derive(Debug, PartialEq, Eq)]
pub enum BinaryTree<T> {
    Nil,
    Node {
        val: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}
impl<T> BinaryTree<T> {
    /// self の Node または Nil を、 to に置き換える。
    /// to は self に組み込まれる形で move される。
    pub fn replace(&mut self, to: Self) {
        *self = to;
    }

    /// self の Node (または Nil) を Nil に置き換える。
    pub fn remove(&mut self) {
        self.replace(BinaryTree::Nil);
    }
}

#[macro_export]
macro_rules! bin_tree {
    ( val: $val:expr, left: $left:expr, right: $right:expr $(,)* ) => {
        BinaryTree::Node {
            val: $val,
            left: Box::new($left),
            right: Box::new($right),
        }
    };

    ( val: $val:expr, right: $right:expr $(,)* ) => {
        bin_tree! {
            val: $val,
            left: bin_tree!(),
            right: $right,
        }
    };

    ( val: $val:expr, left: $left:expr $(,)* ) => {
        bin_tree! {
            val: $val,
            left: $left,
            right: bin_tree!(),
        }
    };

    ( val: $val:expr $(,)* ) => {
        bin_tree!(val: $val, left: bin_tree!(), right: bin_tree!(),)
    };

    () => {
        BinaryTree::Nil
    };
}

#[test]
fn test_replace() {
    // tree1:
    //       5
    //      /
    //     4
    //    /
    //   11
    //  /  \
    // 7    2
    //
    // tree2:
    //         8
    //        / \
    //       13  4
    //            \
    //             1
    //
    // tree3 = tree1.root.right + tree2:
    //       5
    //      / \
    //     4   8
    //    /   / \
    //   11  13  4
    //  /  \      \
    // 7    2      1
    //

    // tree1 は後ほどルートの右のNilを置き換えるので、 mut でつくる。
    let mut tree1 = bin_tree! {
        val: 5,
        left: bin_tree! {
            val: 4,
            left: bin_tree! {
                val: 11,
                left: bin_tree! { val: 7 },
                right: bin_tree! { val: 2 },
            },
        },
    };

    let tree2 = bin_tree! {
        val: 8,
        left: bin_tree! { val: 13 },
        right: bin_tree! {
            val: 4,
            right: bin_tree! { val: 1 },
        },
    };

    let tree3 = bin_tree! {
        val: 5,
        left: bin_tree! {
            val: 4,
            left: bin_tree! {
                val: 11,
                left: bin_tree! { val: 7 },
                right: bin_tree! { val: 2 },
            },
        },
        right: bin_tree! {
            val: 8,
            left: bin_tree! { val: 13 },
            right: bin_tree! {
                val: 4,
                right: bin_tree!{ val: 1 },
            },
        },
    };

    if let BinaryTree::Node { right, .. } = &mut tree1 {
        // tree1のルートの右を、Nilからtree2のルートに置き換える。
        //
        // 型の解説:
        //   right: &mut Box<BinaryTree>
        //   *right: mut Box<BinaryTree>
        //   **right: mut BinaryTree
        //
        // replaceは &mut BinaryTree をセルフとして受け取るので (&mut **right).replace と書くのが明示的だが、
        // `.` 演算子が暗黙的に借用への変換を行ってくれる。
        (**right).replace(tree2);
    }
    assert_eq!(&tree1, &tree3);
}
