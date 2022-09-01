// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn f(node: &Rc<RefCell<TreeNode>>, maxv: i32) -> u32 {
            let node = node.borrow();
            (if maxv <= node.val { 1 } else { 0 })
                + (if let Some(n) = &node.left {
                    f(n, maxv.max(node.val))
                } else {
                    0
                })
                + (if let Some(n) = &node.right {
                    f(n, maxv.max(node.val))
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
