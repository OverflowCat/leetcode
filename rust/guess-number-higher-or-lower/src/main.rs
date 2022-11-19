/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut mid = n / 2;
        let mut upper = n;
        let mut lower = 1;
        loop {
            match guess(mid) {
                1 => {
                    lower = mid + 1;
                    mid += (upper - mid + 1) / 2;
                }
                -1 => {
                    upper = mid - 1;
                    mid -= (mid - lower + 1) / 2;
                }
                _ => return mid,
            }
        }
    }
}
