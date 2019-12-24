use data_structures_and_algorithms_rs::BinaryTree;

#[test]
fn leetcode_112_path_sum_dfs() {
    use BinaryTree::{Nil, Node};

    // この関数を書き上げることが問題に対する回答。
    // 二分木のルートと、目指す総和を入力に取り、総和を実現する path が存在するかどうかを返却。
    pub fn has_path_sum(root: &BinaryTree<i32>, sum: i32) -> bool {
        // 再帰のヘルパー関数。「ルートから cur_node までたどってきました。今までの総和は cur_sum です。」という情報を引数に持つ。
        fn rec(cur_node: &BinaryTree<i32>, cur_sum: i32, sum: i32) -> bool {
            match cur_node {
                Nil => panic!("cur_node が Nil のときは呼び出さないでください"),

                // パターンマッチで、 cur_node の値、左右の子ノードを一気に束縛できる。
                Node { val, left, right } => {
                    let cur_sum = cur_sum + val;

                    // left の型は &Box<BinaryTree> となっている。& は cur_node が参照なので、 `match cur_node` で付いた。
                    // *left で Box<BinaryTree> となる。このままだと Box が邪魔で取り回しづらいので、
                    // **left で BinaryTree 型とする。ただしこれをそのまま使うと所有権が移動してしまうので、
                    // &(**left) として &BinaryTree 型を得る。
                    //
                    // ここは boxキーワード (https://doc.rust-jp.rs/the-rust-programming-language-ja/1.9/book/box-syntax-and-patterns.html) を使えばもっときれいに書ける。
                    match (&(**left), &(**right)) {
                        (Nil, Nil) => cur_sum == sum, // Leafに到達しpathが完成したので、sumと比較
                        (_, Nil) => rec(&(*left), cur_sum, sum), // 左の子があるので path 未完成。左の子も辿る。
                        (Nil, _) => rec(&(*right), cur_sum, sum),
                        (_, _) => rec(&(*left), cur_sum, sum) || rec(&(*right), cur_sum, sum), // 左右の子どちらかが総和を満たす path を持っていれば十分。
                    }
                }
            }
        }

        // 探索開始
        rec(root, 0, sum)
    }

    // 二分木の構築。
    //
    //       5
    //      / \
    //     4   8
    //    /   / \
    //   11  13  4
    //  /  \      \
    // 7    2      1
    //
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

    // 総和が22になるpathは存在する。
    //
    //      *5*
    //      / \
    //    *4*   8
    //    /   / \
    //  *11*  13  4
    //  /  \      \
    // 7   *2*      1
    //
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

    // この関数を書き上げることが問題に対する回答。
    // 二分木（サブツリー）のルートを入力に取り、左右の子ノードを再帰的に prune_tree() していく。
    // 自分自身が（左右の子を prune した結果、または最初から）リーフであった場合には、自分の値が0ならば、自分自身をNilに置き換える。
    // in-place で二分木を書き換えていく。
    pub fn prune_tree(root: &mut Box<BinaryTree<i32>>) {
        // 型の解説:
        //   root: &mut Box<BinaryTree>
        //   *root: mut Box<BinaryTree>
        //   **root: mut BinaryTree
        //   &mut **root: &mut BinaryTree
        match &mut **root {
            Nil => {}
            Node { val, left, right } => match (&mut **left, &mut **right) {
                (Nil, Nil) => {
                    // root 自身がリーフ
                    if *val == 0 {
                        root.remove(); // root を Nil に書き換える
                    }
                }
                (_, Nil) => {
                    prune_tree(left);
                    if let Nil = &mut **left {
                        // 左の子ノードを prune した結果、rootがリーフになった
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

    // 二分木の構築。
    //
    //       1
    //      / \
    //     /   \
    //    0     1
    //   / \   / \
    //  /   | |   \
    // 0    0 0    1
    //
    let mut tree = Box::new(BinaryTree::<i32>::Node {
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

    // prune_tree(tree) した結果の期待値。
    //
    //  1
    //   \
    //    1
    //     \
    //      1
    //
    let pruned = BinaryTree::<i32>::Node {
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
            prune_tree(&mut tree);
            *tree
        },
        pruned
    );
}
