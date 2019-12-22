use std::cell::RefCell;
use std::rc::Rc;

// LeetCodeのデフォルト。in-placeで値を書き換えられるのはいいけど、RefCell使うせいで参照チェック（borrow_mut が1人だけかとか）がruntimeになるのが厳しい
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode_Rc_RefCell {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode_Rc_RefCell>>>,
    pub right: Option<Rc<RefCell<TreeNode_Rc_RefCell>>>,
}

// 読み出しだけならこれで十分。書き込みもこれでできる
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode_Box {
    pub val: i32,
    pub left: Option<Box<TreeNode_Box>>, // Boxじゃないとサイズが定まらない
    pub right: Option<Box<TreeNode_Box>>,
}

// 記事で使う
#[derive(Debug, PartialEq, Eq)]
pub enum BinaryTree<T> {
    Nil,
    Node {
        val: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

// in-placeで書き換えられるし、compile time な borrow checkができる？
// pub struct TreeNode<'r, 'l, 'r> {
//     pub val: i32,
//     pub left: Option<&'l mut TreeNode<'l>>,
//     pub right: Option<&'r mut TreeNode<'r>>,
// }
// 全部 'a は制約が強すぎる。「再帰的に子供を作る」みたいなときに、子は親よりもlifetimeが短くなっちゃう。
// じゃあ子のlieftimeを分けよう！と思っても、孫のlifetimeも分からないといけないので、これは作れない

// サポートしたい演算
// df_iter
// bf_iter
// append_left, append_right
// cut_left, cut_right
