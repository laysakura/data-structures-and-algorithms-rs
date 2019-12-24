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

#[test]
fn replace() {
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
        left: Box::new(BinaryTree::<i32>::Node {
            val: 4,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 11,
                left: Box::new(BinaryTree::<i32>::Node {
                    val: 7,
                    left: Box::new(Nil),
                    right: Box::new(Nil),
                }),
                right: Box::new(BinaryTree::<i32>::Node {
                    val: 2,
                    left: Box::new(Nil),
                    right: Box::new(Nil),
                }),
            }),
            right: Box::new(Nil),
        }),
        right: Box::new(Nil),
    };

    let tree2 = BinaryTree::<i32>::Node {
        val: 8,
        left: Box::new(BinaryTree::<i32>::Node {
            val: 13,
            left: Box::new(Nil),
            right: Box::new(Nil),
        }),
        right: Box::new(BinaryTree::<i32>::Node {
            val: 4,
            left: Box::new(Nil),
            right: Box::new(BinaryTree::<i32>::Node {
                val: 1,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
        }),
    };

    let tree3 = BinaryTree::<i32>::Node {
        val: 5,
        left: Box::new(BinaryTree::<i32>::Node {
            val: 4,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 11,
                left: Box::new(BinaryTree::<i32>::Node {
                    val: 7,
                    left: Box::new(Nil),
                    right: Box::new(Nil),
                }),
                right: Box::new(BinaryTree::<i32>::Node {
                    val: 2,
                    left: Box::new(Nil),
                    right: Box::new(Nil),
                }),
            }),
            right: Box::new(Nil),
        }),
        right: Box::new(BinaryTree::<i32>::Node {
            val: 8,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 13,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
            right: Box::new(BinaryTree::<i32>::Node {
                val: 4,
                left: Box::new(Nil),
                right: Box::new(BinaryTree::<i32>::Node {
                    val: 1,
                    left: Box::new(Nil),
                    right: Box::new(Nil),
                }),
            }),
        }),
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
