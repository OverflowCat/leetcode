struct Solution {}

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut count = 0;
        for i in 0..2u32.pow(arr.len() as u32) {
            let mut used = [false; 26];
            let mut deb = String::new();
            (0..32 - i.leading_zeros() as usize)
                .filter(|j| i >> j & 1 == 1)
                .all(|j| {
                    arr[j].bytes().all(|x| {
                        let letter = (x - b'a') as usize;
                        if used[letter] {
                            return false;
                        }
                        used[letter] = true;
                        deb.push(x as char);
                        return true;
                    })
                })
                .then(|| {
                    println!("{}", deb);
                    count += 1
                });
        }
        count
    }
}

fn main() {
    let s = vec![String::from("un"), String::from("iq"), String::from("ue")];
    let a = Solution::max_length(s);
    println!("{}", a);
}
