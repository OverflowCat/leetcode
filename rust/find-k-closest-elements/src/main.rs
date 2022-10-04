use std::cmp::Ordering::*;
impl Solution {
    #[inline]
    pub fn compare(a: &i32, b: &i32, x: &i32) -> bool {
        let left_part = (a - x).abs();
        let right_part = (b - x).abs();
        match left_part.cmp(&right_part) {
            Less => true,
            Equal => a < b,
            _ => false,
        }
    }

    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let last_i = arr.len() - 1;
        if last_i == 0 {
            return arr;
        }
        if k == 1 {
            return match arr.binary_search(&x) {
                Ok(_) => {
                    vec![x]
                }
                Err(mut i) => {
                    if i > last_i {
                        i = last_i;
                    }
                    if i > 0 && i < last_i {
                        let (left, mid, right) = (arr[i - 1], arr[i], arr[i + 1]);
                        let x = if Solution::compare(&left, &mid, &x) {
                            left
                        } else {
                            mid
                        };
                        vec![if Solution::compare(&mid, &right, &x) {
                            mid
                        } else {
                            right
                        }]
                    } else {
                        let (mid, another) = (arr[i], arr[if i > 0 { i - 1 } else { i + 1 }]);
                        vec![if Solution::compare(&mid, &another, &x) {
                            mid
                        } else {
                            another
                        }]
                    }
                }
            };
        }
        let k = k as usize;
        let (mut l, mut r) = match arr.binary_search(&x) {
            Ok(i) => {
                if i < last_i && i > 0 {
                    if Solution::compare(&arr[i - 1], &arr[i + 1], &x) {
                        (i - 1, i)
                    } else {
                        (i, i + 1)
                    }
                } else if i > 0 {
                    (i - 1, i)
                } else {
                    (i, i + 1)
                }
            }
            Err(mut i) => {
                if i > last_i {
                    i = last_i;
                }
                if i > 0 && i < last_i {
                    let a = arr[i - 1];
                    let b = arr[i];
                    let c = arr[i + 1];
                    let ab = Solution::compare(&a, &b, &x);
                    let bc = Solution::compare(&b, &c, &x);
                    if ab && bc {
                        (i - 1, i)
                    } else {
                        (i, i + 1)
                    }
                } else if i > 0 {
                    (i - 1, i)
                } else {
                    (i, i + 1)
                }
            }
        };
        while r - l + 1 < k {
            println!("l: {}, r: {}", l, r);
            let (left, right) = (arr[l], arr[r]);
            if Self::compare(&left, &right, &x) {
                if l > 0 {
                    l -= 1;
                } else {
                    r += 1;
                }
            } else {
                if r < last_i {
                    r += 1;
                } else {
                    l -= 1;
                }
            }
        }
        arr[l..=r].to_vec()
    }
}

struct Solution;

fn main() {
    // Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3);
    // Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1);
    // println!("{:?}", Solution::find_closest_elements(vec![1, 2], 1, 3));
    println!(
        "{:?}",
        Solution::find_closest_elements(vec![0, 0, 1, 2, 3, 3, 4, 7, 7, 8], 3, 5)
    );
}
