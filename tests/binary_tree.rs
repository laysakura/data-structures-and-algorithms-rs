use data_structures_and_algorithms_rs::{BinaryTree, TreeNode_Box, TreeNode_Rc_RefCell};

#[test]
fn leetcode_112_path_sum_BinaryTree() {
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
    assert_eq!(has_path_sum(&root, 23), true);
}

#[test]
fn leetcode_814_binary_tree_pruning_BinaryTree() {
    use BinaryTree::{Nil, Node};

    pub fn prune_tree(root: &mut Box<BinaryTree<i32>>) {
        match &mut **root {
            Nil => {}
            Node { val, left, right } => match (&mut **left, &mut **right) {
                (Nil, Nil) => {
                    if *val == 0 {
                        *root = Box::new(Nil);
                    }
                }
                (_, Nil) => {
                    prune_tree(left);
                    if let Nil = &mut **left {
                        if *val == 0 {
                            *root = Box::new(Nil);
                        }
                    }
                }
                (Nil, _) => {
                    prune_tree(right);
                    if let Nil = &mut **right {
                        if *val == 0 {
                            *root = Box::new(Nil);
                        }
                    }
                }
                (_, _) => {
                    prune_tree(left);
                    prune_tree(right);
                    if let (Nil, Nil) = (&mut **left, &mut **right) {
                        if *val == 0 {
                            *root = Box::new(Nil);
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

// ----------------- 以下、試行錯誤 --------------------------------
#[test]
fn leetcode_112_path_sum_TreeNode_Rc_RefCell() {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode_Rc_RefCell>>>, sum: i32) -> bool {
        fn rec(root: Option<Rc<RefCell<TreeNode_Rc_RefCell>>>, cur_sum: i32, sum: i32) -> bool {
            match root {
                None => panic!(),
                Some(root) => {
                    let root = root.borrow();
                    let cur_sum = cur_sum + root.val;
                    println!("{:?}", root.val);

                    match (root.left.clone(), root.right.clone()) {
                        (None, None) => cur_sum == sum,
                        (Some(_), None) => rec(root.left.clone(), cur_sum, sum),
                        (None, Some(_)) => rec(root.right.clone(), cur_sum, sum),
                        (Some(_), Some(_)) => {
                            rec(root.left.clone(), cur_sum, sum)
                                || rec(root.right.clone(), cur_sum, sum)
                        }
                    }
                }
            }
        }

        match root {
            None => false,
            Some(_) => rec(root, 0, sum),
        }
    }

    let root = TreeNode_Rc_RefCell {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 13,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    };
    assert_eq!(has_path_sum(None, 22), false);
    assert_eq!(has_path_sum(Some(Rc::new(RefCell::new(root))), 22), true);
}

#[test]
fn leetcode_112_path_sum_TreeNode_Box() {
    pub fn has_path_sum(root: &Option<Box<TreeNode_Box>>, sum: i32) -> bool {
        fn rec(root: &Option<Box<TreeNode_Box>>, cur_sum: i32, sum: i32) -> bool {
            match root {
                None => panic!(),
                Some(root) => {
                    let cur_sum = cur_sum + root.val;
                    println!("{:?}", root.val);

                    match (&root.left, &root.right) {
                        (None, None) => cur_sum == sum,
                        (Some(_), None) => rec(&root.left, cur_sum, sum),
                        (None, Some(_)) => rec(&root.right, cur_sum, sum),
                        (Some(_), Some(_)) => {
                            rec(&root.left, cur_sum, sum) || rec(&root.right, cur_sum, sum)
                        }
                    }
                }
            }
        }

        match root {
            None => false,
            Some(_) => rec(root, 0, sum),
        }
    }

    let root = TreeNode_Box {
        val: 5,
        left: Some(Box::new(TreeNode_Box {
            val: 4,
            left: Some(Box::new(TreeNode_Box {
                val: 11,
                left: Some(Box::new(TreeNode_Box {
                    val: 7,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode_Box {
                    val: 2,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        })),
        right: Some(Box::new(TreeNode_Box {
            val: 8,
            left: Some(Box::new(TreeNode_Box {
                val: 13,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode_Box {
                val: 4,
                left: None,
                right: Some(Box::new(TreeNode_Box {
                    val: 1,
                    left: None,
                    right: None,
                })),
            })),
        })),
    };
    assert_eq!(has_path_sum(&None, 22), false);
    assert_eq!(has_path_sum(&Some(Box::new(root)), 22), true);
}

#[test]
fn aaa() {
    fn move_string(s: String) -> String {
        s
    }

    let s = "abc".to_string();

    println!("{}", s);
    println!("{}", move_string(s));
}

#[test]
fn leetcode_814_binary_tree_pruning_TreeNode_Rc_RefCell() {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn prune_tree(
        root: Option<Rc<RefCell<TreeNode_Rc_RefCell>>>,
    ) -> Option<Rc<RefCell<TreeNode_Rc_RefCell>>> {
        // in-placeで変えちゃう

        match root.clone() {
            None => None,
            Some(root_) => {
                let mut root_r = root_.borrow_mut();
                match (root_r.left.clone(), root_r.right.clone()) {
                    (None, None) => {
                        if root_r.val == 0 {
                            None
                        } else {
                            root
                        }
                    }
                    (Some(_), None) => {
                        root_r.left = prune_tree(root_r.left.clone());
                        match root_r.left.clone() {
                            None => {
                                if root_r.val == 0 {
                                    None
                                } else {
                                    root
                                }
                            }
                            _ => root,
                        }
                    }
                    (None, Some(_)) => {
                        root_r.right = prune_tree(root_r.right.clone());
                        match root_r.right.clone() {
                            None => {
                                if root_r.val == 0 {
                                    None
                                } else {
                                    root
                                }
                            }
                            _ => root,
                        }
                    }
                    (Some(_), Some(_)) => {
                        root_r.left = prune_tree(root_r.left.clone());
                        root_r.right = prune_tree(root_r.right.clone());
                        match (root_r.left.clone(), root_r.right.clone()) {
                            (None, None) => {
                                if root_r.val == 0 {
                                    None
                                } else {
                                    root
                                }
                            }
                            _ => root,
                        }
                    }
                }
            }
        }
    }

    let tree1 = TreeNode_Rc_RefCell {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 1,
                left: None,
                right: None,
            }))),
        }))),
    };
    let pruned1 = TreeNode_Rc_RefCell {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 1,
                left: None,
                right: None,
            }))),
        }))),
    };

    let tree2 = TreeNode_Rc_RefCell {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 0,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 1,
                left: None,
                right: None,
            }))),
        }))),
    };
    let pruned2 = TreeNode_Rc_RefCell {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode_Rc_RefCell {
                val: 1,
                left: None,
                right: None,
            }))),
        }))),
    };

    assert_eq!(
        prune_tree(Some(Rc::new(RefCell::new(tree1)))),
        Some(Rc::new(RefCell::new(pruned1)))
    );
    assert_eq!(
        prune_tree(Some(Rc::new(RefCell::new(tree2)))),
        Some(Rc::new(RefCell::new(pruned2)))
    );
}

#[test]
fn leetcode_814_binary_tree_pruning_TreeNode_Box() {
    pub fn prune_tree(root: &mut Option<Box<TreeNode_Box>>) {
        // in-placeで変えちゃう

        match root {
            None => {}
            Some(ref mut root_box) => match (&mut root_box.left, &mut root_box.right) {
                (None, None) => {
                    if root_box.val == 0 {
                        *root = None;
                    }
                }
                (Some(_), None) => {
                    prune_tree(&mut root_box.left);
                    if root_box.left.is_none() && root_box.val == 0 {
                        *root = None;
                    }
                }
                (None, Some(_)) => {
                    prune_tree(&mut root_box.right);
                    if root_box.right.is_none() && root_box.val == 0 {
                        *root = None;
                    }
                }
                (Some(_), Some(_)) => {
                    prune_tree(&mut root_box.left);
                    prune_tree(&mut root_box.right);
                    if root_box.left.is_none() && root_box.right.is_none() && root_box.val == 0 {
                        *root = None;
                    }
                }
            },
        }
    }

    let mut tree1 = Some(Box::new(TreeNode_Box {
        val: 1,
        left: None,
        right: Some(Box::new(TreeNode_Box {
            val: 0,
            left: Some(Box::new(TreeNode_Box {
                val: 0,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode_Box {
                val: 1,
                left: None,
                right: None,
            })),
        })),
    }));
    let pruned1 = TreeNode_Box {
        val: 1,
        left: None,
        right: Some(Box::new(TreeNode_Box {
            val: 0,
            left: None,
            right: Some(Box::new(TreeNode_Box {
                val: 1,
                left: None,
                right: None,
            })),
        })),
    };

    let mut tree2 = Some(Box::new(TreeNode_Box {
        val: 1,
        left: Some(Box::new(TreeNode_Box {
            val: 0,
            left: Some(Box::new(TreeNode_Box {
                val: 0,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode_Box {
                val: 0,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode_Box {
            val: 1,
            left: Some(Box::new(TreeNode_Box {
                val: 0,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode_Box {
                val: 1,
                left: None,
                right: None,
            })),
        })),
    }));
    let pruned2 = TreeNode_Box {
        val: 1,
        left: None,
        right: Some(Box::new(TreeNode_Box {
            val: 1,
            left: None,
            right: Some(Box::new(TreeNode_Box {
                val: 1,
                left: None,
                right: None,
            })),
        })),
    };

    assert_eq!(
        {
            prune_tree(&mut tree1);
            *tree1.unwrap()
        },
        pruned1
    );
    assert_eq!(
        {
            prune_tree(&mut tree2);
            *tree2.unwrap()
        },
        pruned2
    );
}
