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
}

struct Solution;

impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut head) = head {
            let mut i = head.as_ref();
            let mut j = head.as_ref().next.as_ref().unwrap().as_ref();
            while j.next.is_some() {
                j = j.next.as_ref().unwrap().as_ref();
                if j.next.is_none() {
                    break;
                }
                i = i.next.as_ref().unwrap().as_ref();
                j = j.next.as_ref().unwrap().as_ref();
            }
            i.next = i.next.as_mut()
            .unwrap()
            .next
            .as_ref();
            Some(head)
        } else {
            None
        }
    }
}
