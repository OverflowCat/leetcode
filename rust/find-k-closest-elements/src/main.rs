use std::cmp::Ordering::*;
impl Solution {
    #[inline]
    pub fn compare(a: &i32, b: &i32) {
        
    }

    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let last_i = arr.len() - 1;
        if last_i == 0 {
            return arr;
        }
            if k == 0 {
                   return match arr.binary_search(&x) {
                       Ok(i) => {
                           vec![x]
                       }
                       Err(i) => {
                           if i > 0 && i < last_i {
                               let (left, right) = arr[i - 1], arr[i + 1];

                           } else {

                           }
                       }
                   };
               }
        let k = k as usize;
        let (mut l, mut r) = match arr.binary_search(&x) {
            Ok(i) | Err(i) => {
                if i < last_i {
                    (i, i + 1)
                } else {
                    (i - 1, i)
                }
            }
        };
        while r - l < k {
            println!("l: {}, r: {}", l, r);
            let (left, right) = (arr[l], arr[r]);
            let a = (left - x).abs();
            let b = (right - x).abs();
            match a.cmp(&b) {
                Greater => {
                    if l > 0 {
                        l -= 1;
                    } else {
                        r += 1;
                    }
                }
                Equal => {
                    if left < right && l > 0 {
                        l -= 1
                    } else if r < last_i {
                        r += 1
                    }
                }
                Less => {
                    if r < last_i {
                        r += 1
                    } else {
                        l -= 1
                    }
                }
            }
        }
        println!("{}, {}", l, r);
        arr[l..=r].to_vec()
    }
}

struct Solution;

fn main() {
    Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3);
    Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1);
}
