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
macro_rules! binary_tree_node {
    (val: $val:expr, left: $left:expr, right: $right:expr,) => (
        (
            Box::new(BinaryTree::Node {
                val: $val,
                left: $left,
                right: $right,
            })
        )
    );

    (val: $val:expr, right: $right:expr,) => (
        (
            binary_tree_node! {
                val: $val,
                left: Box::new(BinaryTree::Nil),
                right: $right,
            }
        )
    );

    (val: $val:expr, left: $left:expr,) => (
        (
            binary_tree_node! {
                val: $val,
                left: $left,
                right: Box::new(BinaryTree::Nil),
            }
        )
    );

    (val: $val: expr) => (
        binary_tree_node!(
            val: $val,
            left: Box::new(BinaryTree::Nil),
            right: Box::new(BinaryTree::Nil),
        )
    );
}


#[test]
fn test_replace() {
    use BinaryTree::{Nil, Node};

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
    let mut tree1 = BinaryTree::<i32>::Node {
        val: 5,
        left: binary_tree_node! {
            val: 4,
            left: binary_tree_node! {
                val: 11,
                left: binary_tree_node! { val: 7 },
                right: binary_tree_node! { val: 2 },
            },
        },
        right: Box::new(Nil),
    };

    let tree2 = BinaryTree::<i32>::Node {
        val: 8,
        left: binary_tree_node! { val: 13 },
        right: binary_tree_node! {
            val: 4,
            right: binary_tree_node! { val: 1 },
        },
    };

    let tree3 = BinaryTree::<i32>::Node {
        val: 5,
        left: binary_tree_node! {
            val: 4,
            left: binary_tree_node! {
                val: 11,
                left: binary_tree_node! { val: 7 },
                right: binary_tree_node! { val: 2 },
            },
        },
        right: binary_tree_node! {
            val: 8,
            left: binary_tree_node! { val: 13 },
            right: binary_tree_node! {
                val: 4,
                right: binary_tree_node!{ val: 1 },
            },
        },
    };

    if let Node {
        val: _,
        left: _,
        right,
    } = &mut tree1
    {
        (**right).replace(tree2); // tree1のルートの右を、Nilからtree2のルートに置き換える
    }
    assert_eq!(&tree1, &tree3);
}
