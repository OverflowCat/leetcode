use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn f(node: &Rc<RefCell<TreeNode>>, maxv: i32) -> u32 {
            let node = node.borrow();
            let mut maxv = maxv;
            (if maxv <= node.val {
                maxv = maxv.max(node.val);
                1
            } else {
                0
            }) + (if let Some(n) = &node.left {
                f(n, maxv)
            } else {
                0
            }) + (if let Some(n) = &node.right {
                f(n, maxv)
            } else {
                0
            })
        }
        if let Some(node) = &root {
            f(node, node.borrow().val) as i32
        } else {
            0
        }
    }
}
