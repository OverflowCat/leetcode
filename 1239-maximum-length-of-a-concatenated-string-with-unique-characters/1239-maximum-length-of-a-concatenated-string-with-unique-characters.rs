impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut mlen = 0;
        for i in 0..2u32.pow(arr.len() as u32) {
            let mut used = [false; 26];
            let mut clen = 0;
            (0..32 - i.leading_zeros() as usize)
                .filter(|j| i >> j & 1 == 1)
                .all(|j| {
                    arr[j].bytes().all(|x| {
                        let letter = (x - b'a') as usize;
                        if used[letter] {
                            return false;
                        }
                        used[letter] = true;
                        clen += 1;
                        return true;
                    })
                })
                .then(|| mlen = mlen.max(clen));
        }
        mlen
    }
}