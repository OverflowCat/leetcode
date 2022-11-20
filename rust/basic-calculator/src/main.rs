#[derive(Debug)]
enum Part {
    Num(i32),
    L,
    R,
    P,
    M,
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        use Part::*;
        let mut expression = vec![L];
        let mut num = 0;
        s.as_bytes().into_iter().for_each(|c| {
            println!("{}", *c as char);
            if let Some(x) = match *c {
                digit if b'0' <= digit && digit <= b'9' => {
                    num *= 10;
                    num += (digit - b'0') as i32;
                    None
                }
                b'+' => Some(P),
                b'-' => Some(M),
                b'(' => Some(L),
                b')' => Some(R),
                _ => None,
            } {
                if num != 0 {
                    expression.push(Num(num));
                }
                expression.push(x);
            }
        });
        expression.push(R);
        fn calc(e: &[Part]) -> (i32, usize) {
            println!("Calculating {:?}", e);
            let mut res = 0;
            let mut positive = 1;
            let mut i = 0;
            loop {
                match e[i] {
                    M => {
                        positive *= -1;
                    }
                    Num(x) => {
                        res += x * positive;
                        positive = 1;
                    }
                    R => {
                        break (res, i);
                    }
                    L => {
                        let (subres, subr_index) = calc(&e[i + 1..]);
                        res += subres * positive;
                        i += subr_index;
                        positive = 1;
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        calc(&expression).0
    }
}

// 1 + ( 2 + 3 ) + 4
struct Solution {}

fn main() {
    assert_eq!(Solution::calculate("1 + 1".into()), 2);
}
