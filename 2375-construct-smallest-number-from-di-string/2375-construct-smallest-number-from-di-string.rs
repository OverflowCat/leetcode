impl Solution {
    pub fn smallest_number(mut pattern: String) -> String {
        pattern.push('X');
        let length = pattern.len();
        let mut res: Vec<u8> = (1..=length as u8).collect();
        let partial_rev = |v: &mut Vec<u8>, mut i: usize, mut j: usize| {
            while i < j {
                v.swap(i, j);
                i += 1;
                j -= 1;
            }
        };
        let mut d_start: Option<usize> = None;
        pattern
            .bytes()
            .enumerate()
            .for_each(|(i, b)| match (d_start, b) {
                (None, b'D') => {
                    d_start = Some(i);
                }
                (_, b'D') => {}
                (Some(start), _) => {
                    partial_rev(&mut res, start, i);
                    d_start = None;
                }
                _ => {}
            });
        res.iter().map(|x| x.to_string()).collect()
    }
}
