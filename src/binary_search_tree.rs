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
pub struct BinarySearchTree<T: Ord>(BinarySearchTreeInner<T>);

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
        let nil = Self::find_nil_to_add(&mut self.0, &val);

        // ノード値が val であるようなリーフで置き換える。
        *nil = BinarySearchTreeInner::Node {
            val,
            left: Box::new(BinarySearchTreeInner::Nil),
            right: Box::new(BinarySearchTreeInner::Nil),
        }
    }

    /// val を二分探索木に追加する場合に、val と交換すべき箇所の Nil を、深さ優先探索で探す。
    ///
    /// 上図の二分木の例だと、
    /// - val == 1 の場合: `3` の左の Nil
    /// - val == 5 の場合: リーフの `5` の左の Nil
    /// - val == 16 の場合: `15` の右の Nil
    fn find_nil_to_add<'v, 'bst>(
        cur_node: &'bst mut BinarySearchTreeInner<T>,
        val: &'v T,
    ) -> &'bst mut BinarySearchTreeInner<T> {
        match cur_node {
            BinarySearchTreeInner::Nil => cur_node,
            BinarySearchTreeInner::Node {
                val: cur_v,
                left,
                right,
            } => {
                if val <= cur_v {
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
            BinarySearchTreeInner::Nil => false,
            BinarySearchTreeInner::Node {
                val: cur_v,
                left,
                right,
            } => {
                if cur_v == val {
                    true
                } else {
                    Self::contains_inner(left, val) || Self::contains_inner(right, val)
                }
            }
        }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    /// 本当はイテレータを返却するほうが、一気に O(n) のメモリか確保する必要がなくて良いが、
    /// イテレータ自体の実装で煩雑にならないようにするために &[T] を返す。
    pub fn get_all_sorted(&self) -> Vec<&T> {
        let mut ret = Vec::<&T>::new();
        Self::get_all_sorted_inner(&self.0, &mut ret);
        ret
    }

    /// in-place order で ret にノード値を追加していく
    fn get_all_sorted_inner<'bst, 'a>(
        cur_node: &'bst BinarySearchTreeInner<T>,
        ret: &'a mut Vec<&'bst T>,
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

#[derive(Debug, PartialEq, Eq)]
enum BinarySearchTreeInner<T: Ord> {
    Nil,
    Node {
        val: T,
        left: Box<Self>,
        right: Box<Self>,
    },
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
