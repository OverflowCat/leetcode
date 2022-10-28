impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let n = img1.len() as i32;
        for x_offset in -29..=29i32 {
            for y_offset in -29..=29i32 {
                let mut count = 0;
                for i in 0.max(0i32 + x_offset)..n.min(n + x_offset) {
                    for j in 0.max(0i32 + y_offset)..n.min(n + y_offset) {
                        if img1[i as usize][j as usize] == 1 && {
                            let (i_, j_) = ((i - x_offset) as usize, (j - y_offset) as usize);
                            img2[i_][j_] == 1
                        } {
                            count += 1;
                        }
                    }
                }
                max = max.max(count);
            }
        }
        max
    }
}

struct Solution {}
