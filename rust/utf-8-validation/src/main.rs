impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut left = 0;
        for c in data {
            match left {
                0 => match c {
                    _ if c < 0b1_0000000 => {}
                    _ if c < 0b11_000000 => {
                        return false;
                    }
                    _ if c < 0b111_00000 => {
                        left += 1;
                    }
                    _ if c < 0b1111_0000 => {
                        left += 2;
                    }
                    _ if c < 0b11111000 => {
                        left += 3;
                    }
                    _ => {
                        return false;
                    }
                },
                _ => {
                    if c >= 0b10000000 && c < 0b11_000000 {
                        if left == 0 {
                            return false;
                        }
                        left -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        left == 0
    }
}
struct Solution {}

fn main() {
    let data = vec![
        194, 155, 231, 184, 185, 246, 176, 131, 161, 222, 174, 227, 162, 134, 241, 154, 168, 185,
        218, 178, 229, 187, 139, 246, 178, 187, 139, 204, 146, 225, 148, 179, 245, 139, 172, 134,
        193, 156, 233, 131, 154, 240, 166, 188, 190, 216, 150, 230, 145, 144, 240, 167, 140, 163,
        221, 190, 238, 168, 139, 241, 154, 159, 164, 199, 170, 224, 173, 140, 244, 182, 143, 134,
        206, 181, 227, 172, 141, 241, 146, 159, 170, 202, 134, 230, 142, 163, 244, 172, 140, 191,
    ];
    println!("{}", Solution::valid_utf8(data));
}

/*
    match String::from_utf8(data.into_iter().map(|x| x as u8).collect()) {
    Ok(x) => {
        println!("{}", x);
        true
    }
    Err(e) => {
        println!("{}", e);
        false
    }
} */
