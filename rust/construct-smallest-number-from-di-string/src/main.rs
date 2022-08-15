fn main() {
    println!("{}", smallest_number("IIIDIDDD".to_string()));
}

fn smallest_number(mut pattern: String) -> String {
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
    for (i, b) in pattern.bytes().enumerate() {
        match b {
            b'D' => {
                if d_start.is_none() {
                    d_start = Some(i);
                }
            }
            _ => {
                if d_start.is_some() {
                    partial_rev(&mut res, d_start.unwrap(), i);
                    d_start = None;
                }
            }
        }
    }
    res.iter().map(|x| x.to_string()).collect()
}

/* fn smallest_number(pattern: String) -> String {
    let mut is_used = [false; 10];
    let mut result: [u8; 9];
    let mut top = 0usize;
    pattern.bytes().for_each(|b| {
        let min_not_used = {
            let mut min = 0;
            for i in b + 1..10 {
                if !is_used[i] {
                    min = i;
                    break;
                }
            }
            if b == 0 {

            }
            min
        };

    });

}
 */
// fn is_match(number: u32, ops: &Vec<bool>) -> bool {}
