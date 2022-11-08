use std::collections::LinkedList;

const DIFF: u8 = b'a' - b'A';

impl Solution {
    pub fn make_good(s: String) -> String {
        let li: LinkedList<u8> = LinkedList::from_iter(s.bytes().into_iter());
        for slice in li.iter_mut().windows(2) {
            if slice[0].abs_diff(slice[1]) == DIFF {
                slice[0]
            }
        }
        s
    }
}

struct Solution {}

fn main() {}
