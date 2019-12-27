/// 二分探索木。
/// あるノードと等しい値は、必ず左側の子ノード以下に入ることとする。
///
/// ```text
///          8
///       __/ \__
///      /       \
///     5        10
///    / \      /  \
///   5   6    9   15
///  /     \
/// 3       8
///  \
///   5
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct BinarySearchTree<T: Ord>(
    // private に実体を持つ。enumを直接使った構築はできない。
    BinarySearchTreeInner<T>,
);

/// 実体。二分木と同じフィールド。
#[derive(Debug, PartialEq, Eq)]
enum BinarySearchTreeInner<T: Ord> {
    Nil,
    Node {
        val: T,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<T: Ord> BinarySearchTree<T> {
    /// 空の二分探索木をつくる。
    pub fn new() -> Self {
        Self(BinarySearchTreeInner::Nil)
    }
}

impl<T: Ord> BinarySearchTree<T> {
    /// 二分探索木に val を追加する。
    /// val は二分探索木に組み込まれる形で move される。
    pub fn add(&mut self, val: T) {
        // val を配置すべきNilを探索。
        let nil = Self::find_nil_to_add(&mut self.0, &val);

        // ノード値が val であるようなリーフを作り、Nilを置き換える。
        *nil = BinarySearchTreeInner::Node {
            val,
            left: Box::new(BinarySearchTreeInner::Nil),
            right: Box::new(BinarySearchTreeInner::Nil),
        }
    }

    /// val を二分探索木に追加する場合に、val と交換すべき箇所の Nil を、深さ優先探索で探す、再帰関数。
    ///
    /// 上図の二分木の例だと、
    /// - val == 1 の場合: `3` の左の Nil
    /// - val == 5 の場合: リーフの `5` の左の Nil
    /// - val == 16 の場合: `15` の右の Nil
    ///
    /// 生存期間パラメータの解説:
    /// - 't : 二分探索木自体の生存期間。 cur_node (現在探索中のノード) も、置き換えるべき Nil も、二分探索木自体の生存期と一致している。
    /// - 'v : 追加する要素の生存期間
    fn find_nil_to_add<'t, 'v>(
        cur_node: &'t mut BinarySearchTreeInner<T>,
        val: &'v T,
    ) -> &'t mut BinarySearchTreeInner<T> {
        match cur_node {
            // Nil まで到達したら、それが val と置き換えるべき Nil。
            BinarySearchTreeInner::Nil => cur_node,

            BinarySearchTreeInner::Node {
                val: cur_v,
                left,
                right,
            } => {
                if val <= cur_v {
                    // 探索中のノード値以下の値を追加したいなら、左に降りる
                    Self::find_nil_to_add(left, &val)
                } else {
                    Self::find_nil_to_add(right, &val)
                }
            }
        }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    /// 二分探索木に val が1つ以上含まれているかを返す。
    pub fn contains(&self, val: &T) -> bool {
        Self::contains_inner(&self.0, &val)
    }

    fn contains_inner(cur_node: &BinarySearchTreeInner<T>, val: &T) -> bool {
        match cur_node {
            BinarySearchTreeInner::Nil => false, // たどってきたpathには val がなかった

            BinarySearchTreeInner::Node {
                val: cur_v,
                left,
                right,
            } => {
                if cur_v == val {
                    // val を見つけたら、リーフまで到達していなくても、これ以上の再起をやめてtrueを返す
                    true
                } else {
                    // 左か右のどちらかに val があるかどうか
                    Self::contains_inner(left, val) || Self::contains_inner(right, val)
                }
            }
        }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn get_all_sorted(&self) -> Vec<&T> {
        let mut ret = Vec::<&T>::new();
        Self::get_all_sorted_inner(&self.0, &mut ret);
        ret
    }

    /// in-place order で ret にノード値を追加していく
    fn get_all_sorted_inner<'t, 'a>(
        cur_node: &'t BinarySearchTreeInner<T>,
        ret: &'a mut Vec<&'t T>,
    ) {
        match cur_node {
            BinarySearchTreeInner::Nil => {}
            BinarySearchTreeInner::Node { val, left, right } => {
                Self::get_all_sorted_inner(left, ret);
                ret.push(val);
                Self::get_all_sorted_inner(right, ret);
            }
        }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn get_range_sorted(&self, min: &T, max: &T) -> Vec<&T> {
        let mut ret = Vec::<&T>::new();
        Self::get_range_sorted_inner(&self.0, min, max, &mut ret);
        ret
    }

    /// in-place order で [min, max] の範囲のノード値を検索し、 ret に追加していく
    fn get_range_sorted_inner<'t, 'a>(
        cur_node: &'t BinarySearchTreeInner<T>,
        min: &T,
        max: &T,
        ret: &'a mut Vec<&'t T>,
    ) {
        match cur_node {
            BinarySearchTreeInner::Nil => {}
            BinarySearchTreeInner::Node { val, left, right } => {
                if val >= min {
                    // cur_node の値が最小値以上なら、まだ左の子ノード以下に最小値以上のノード値があり得るので、探索する。
                    Self::get_range_sorted_inner(left, min, max, ret);
                }
                if min <= val && val <= max {
                    ret.push(val);
                }
                if val < max {
                    // cur_node の値が最大値より小さければ、まだ右の子ノード以下に最大値以下のノード値があり得るので、探索する。
                    Self::get_range_sorted_inner(right, min, max, ret);
                }
            }
        }
    }
}

#[test]
fn add_in_same_order() {
    use BinarySearchTree;

    let mut bst1 = BinarySearchTree::new();
    bst1.add(1);
    bst1.add(2);

    let mut bst2 = BinarySearchTree::new();
    bst2.add(1);
    bst2.add(2);

    assert_eq!(bst1, bst2);
}

#[test]
fn add_in_different_order1() {
    use BinarySearchTree;

    let mut bst1 = BinarySearchTree::new();
    bst1.add(1);
    bst1.add(2);

    let mut bst2 = BinarySearchTree::new();
    bst2.add(2);
    bst2.add(1);

    assert_ne!(bst1, bst2);
}

#[test]
fn add_in_different_order2() {
    use BinarySearchTree;

    let mut bst1 = BinarySearchTree::new();
    bst1.add(8);
    bst1.add(5);
    bst1.add(10);
    bst1.add(5);
    bst1.add(3);
    bst1.add(5);
    bst1.add(6);
    bst1.add(8);
    bst1.add(9);
    bst1.add(15);

    let mut bst2 = BinarySearchTree::new();
    bst2.add(8);
    bst2.add(10);
    bst2.add(5);
    bst2.add(15);
    bst2.add(9);
    bst2.add(6);
    bst2.add(5);
    bst2.add(8);
    bst2.add(3);
    bst2.add(5);

    assert_eq!(bst1, bst2);
}

#[test]
fn contains() {
    let mut bst = BinarySearchTree::new();
    bst.add(8);
    bst.add(5);
    bst.add(10);
    bst.add(5);
    bst.add(3);
    bst.add(5);
    bst.add(6);
    bst.add(8);
    bst.add(9);
    bst.add(15);

    assert_eq!(bst.contains(&0), false);
    assert_eq!(bst.contains(&5), true);
    assert_eq!(bst.contains(&10), true);
    assert_eq!(bst.contains(&9), true);
    assert_eq!(bst.contains(&15), true);
    assert_eq!(bst.contains(&16), false);
}

#[test]
fn get_all_sorted() {
    let mut bst = BinarySearchTree::new();
    bst.add(8);
    bst.add(5);
    bst.add(10);
    bst.add(5);
    bst.add(3);
    bst.add(5);
    bst.add(6);
    bst.add(8);
    bst.add(9);
    bst.add(15);

    assert_eq!(
        bst.get_all_sorted(),
        vec![&3, &5, &5, &5, &6, &8, &8, &9, &10, &15]
    );
}

#[test]
fn get_range_sorted() {
    let mut bst = BinarySearchTree::new();
    bst.add(8);
    bst.add(5);
    bst.add(10);
    bst.add(5);
    bst.add(3);
    bst.add(5);
    bst.add(6);
    bst.add(8);
    bst.add(9);
    bst.add(15);

    assert_eq!(
        bst.get_range_sorted(&3, &15),
        vec![&3, &5, &5, &5, &6, &8, &8, &9, &10, &15]
    );
    assert_eq!(
        bst.get_range_sorted(&5, &15),
        vec![&5, &5, &5, &6, &8, &8, &9, &10, &15]
    );
    assert_eq!(
        bst.get_range_sorted(&5, &14),
        vec![&5, &5, &5, &6, &8, &8, &9, &10]
    );
    assert_eq!(bst.get_range_sorted(&5, &5), vec![&5, &5, &5]);
}
