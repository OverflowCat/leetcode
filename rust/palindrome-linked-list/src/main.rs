fn main() {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(v: Vec<i32>) -> Option<Box<Self>> {
        if v.len() == 0 {
            return None;
        }

        let mut guard_node = Self::new(0);
        {
            let mut curr = &mut guard_node;
            for i in 0..v.len() {
                let mut node = Self::new(v[i]);
                (*curr).next = Some(Box::new(node));
                curr = &mut (*curr).next.as_mut().unwrap();
            }
        }
        guard_node.next
    }
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        false
    }
}
