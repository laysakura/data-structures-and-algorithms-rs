use data_structures_and_algorithms_rs::BinaryTree;

#[test]
fn leetcode_112_path_sum() {
    use BinaryTree::{Nil, Node};

    // この関数を書き上げることが問題に対する回答。
    // 二分木のルートと、目指す総和を入力に取り、
    pub fn has_path_sum(root: &BinaryTree<i32>, sum: i32) -> bool {
        fn rec(cur_node: &BinaryTree<i32>, cur_sum: i32, sum: i32) -> bool {
            match cur_node {
                Nil => panic!("root が Nil のときよ呼び出さないでください"),

                // box を使えばもっときれいになる
                Node { val, left, right } => {
                    let cur_sum = cur_sum + val;

                    match (&(**left), &(**right)) {
                        // Leafに到達しpathが完成したので、sumと比較
                        (Nil, Nil) => cur_sum == sum,

                        (_, Nil) => rec(&(*left), cur_sum, sum),
                        (Nil, _) => rec(&(*right), cur_sum, sum),
                        (_, _) => rec(&(*left), cur_sum, sum) || rec(&(*right), cur_sum, sum),
                    }
                }
            }
        }

        rec(root, 0, sum)
    }

    let root = BinaryTree::<i32>::Node {
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
    assert_eq!(has_path_sum(&root, 22), true);
}

#[test]
fn leetcode_112_path_sum_bfs() {
    use std::collections::VecDeque;
    use BinaryTree::{Nil, Node};

    pub fn has_path_sum(root: &BinaryTree<i32>, sum: i32) -> bool {
        // (今探索中のノード, 親ノードの値のここまでの総和) を格納するキュー
        let mut queue = VecDeque::<(&BinaryTree<i32>, i32)>::new();
        queue.push_back((root, 0));

        while let Some((cur_node, cur_sum)) = queue.pop_front() {
            match cur_node {
                Nil => {
                    panic!("Nil を queue に詰めないでください");
                }
                Node { val, left, right } => {
                    let cur_sum = cur_sum + val;

                    match (&(**left), &(**right)) {
                        // Leafに到達しpathが完成したので、sumと比較
                        (Nil, Nil) => {
                            if cur_sum == sum {
                                return true;
                            }
                        }

                        (_, Nil) => queue.push_back((&(*left), cur_sum)),
                        (Nil, _) => queue.push_back((&(*right), cur_sum)),
                        (_, _) => {
                            queue.push_back((&(*left), cur_sum));
                            queue.push_back((&(*right), cur_sum));
                        }
                    }
                }
            }
        }

        false // キューは空になったが、目指す総和のpathは見つからなかった
    }

    let root = BinaryTree::<i32>::Node {
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
    assert_eq!(has_path_sum(&root, 22), true);
}

#[test]
fn leetcode_814_binary_tree_pruning() {
    use BinaryTree::{Nil, Node};

    pub fn prune_tree(root: &mut Box<BinaryTree<i32>>) {
        match &mut **root {
            Nil => {}
            Node { val, left, right } => match (&mut **left, &mut **right) {
                (Nil, Nil) if *val == 0 => {
                    root.remove();
                }
                (_, Nil) => {
                    prune_tree(left);
                    if let Nil = &mut **left {
                        if *val == 0 {
                            root.remove();
                        }
                    }
                }
                (Nil, _) => {
                    prune_tree(right);
                    if let Nil = &mut **right {
                        if *val == 0 {
                            root.remove();
                        }
                    }
                }
                (_, _) => {
                    prune_tree(left);
                    prune_tree(right);
                    if let (Nil, Nil) = (&mut **left, &mut **right) {
                        if *val == 0 {
                            root.remove();
                        }
                    }
                }
            },
        }
    }

    let mut tree1 = Box::new(BinaryTree::<i32>::Node {
        val: 1,
        left: Box::new(Nil),
        right: Box::new(BinaryTree::<i32>::Node {
            val: 0,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 0,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
            right: Box::new(BinaryTree::<i32>::Node {
                val: 1,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
        }),
    });
    let pruned1 = BinaryTree::<i32>::Node {
        val: 1,
        left: Box::new(Nil),
        right: Box::new(BinaryTree::<i32>::Node {
            val: 0,
            left: Box::new(Nil),
            right: Box::new(BinaryTree::<i32>::Node {
                val: 1,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
        }),
    };

    let mut tree2 = Box::new(BinaryTree::<i32>::Node {
        val: 1,
        left: Box::new(BinaryTree::<i32>::Node {
            val: 0,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 0,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
            right: Box::new(BinaryTree::<i32>::Node {
                val: 0,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
        }),
        right: Box::new(BinaryTree::<i32>::Node {
            val: 1,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 0,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
            right: Box::new(BinaryTree::<i32>::Node {
                val: 1,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
        }),
    });
    let pruned2 = BinaryTree::<i32>::Node {
        val: 1,
        left: Box::new(Nil),
        right: Box::new(BinaryTree::<i32>::Node {
            val: 1,
            left: Box::new(Nil),
            right: Box::new(BinaryTree::<i32>::Node {
                val: 1,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
        }),
    };

    assert_eq!(
        {
            prune_tree(&mut tree1);
            *tree1
        },
        pruned1
    );
    assert_eq!(
        {
            prune_tree(&mut tree2);
            *tree2
        },
        pruned2
    );
}
