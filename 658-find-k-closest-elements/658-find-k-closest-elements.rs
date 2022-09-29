impl Solution{
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut l, mut r, k) = (0, arr.len() - 1, k as usize);
        while r - l + 1 > k {
            if arr[r] - x < x - arr[l] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        arr[l..=r].to_vec()
    }
}
