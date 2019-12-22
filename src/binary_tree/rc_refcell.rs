impl TreeNode_Rc_RefCell {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode_Rc_RefCell {
            val,
            left: None,
            right: None,
        }
    }
}

