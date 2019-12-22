use data_structures_and_algorithms_rs::{TreeNode_Box, TreeNode_Rc_RefCell};

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

        match *root {
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
