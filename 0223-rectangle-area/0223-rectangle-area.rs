impl Solution {
    pub fn compute_area(
        ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let (l, r, t, b) = (ax1.max(bx1), ax2.min(bx2), ay2.min(by2), ay1.max(by1));
        (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1)
            - if r > l && t > b { (r - l) * (t - b) } else { 0 }
    }
}
