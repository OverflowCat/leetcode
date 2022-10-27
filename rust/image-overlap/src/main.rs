impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let n = img1.len() as i32;
        for x_offset in -29..=29 {
            for y_offset in -29..=29 {
                let mut count = 0;
                for i in 0.max(0i32 + x_offset) as usize..n.min(n + x_offset) as usize {
                    
                    for j in 0.max(0i32 + y_offset) as usize..n.min(n + y_offset) as usize {
                        if img1[i][j]  == img2[i][j] {
                            count += 1;
                        }
                    }
                }
            }
        }
        max
    }
}

struct Solution {}
