#[derive(Debug)]
enum Part { Num(i32), L, R, P, M }

impl Solution {
    pub fn calculate(mut s: String) -> i32 {
        use Part::*; let mut expression = vec![L];
        let mut num = 0; let mut num_flag = false;
        s.push(')');
        s.as_bytes().into_iter().for_each(|c| {
            if let Some(x) = match *c {
                digit if b'0' <= digit && digit <= b'9' => {
                    num *= 10; num += (digit - b'0') as i32;
                    num_flag = true; None
                }
                b'+' => Some(P), b'-' => Some(M),
                b'(' => Some(L), b')' => Some(R), _ => None,
            } {
                if num_flag {
                    expression.push(Num(num));
                    num = 0; num_flag = false;
                } expression.push(x);
            }
        });
        fn calc(e: &[Part]) -> (i32, usize) {
            let mut res = 0; let mut positive = 1;
            let mut i = 0; loop {
                match e[i] {
                    M => { positive *= -1; }
                    Num(x) => { res += x * positive; positive = 1; }
                    R => { break (res, i); }
                    L => {
                        i += 1;
                        let (subres, subr_index) = calc(&e[i..]);
                        res += subres * positive;
                        i += subr_index;
                        if i == e.len() - 1 {
                            break (res, i);
                        }
                        positive = 1;
                    }
                    _ => {}
                } i += 1;
            }
        } calc(&expression).0
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::calculate("1 + 1".into()), 2);
    assert_eq!(Solution::calculate("1 + ( 2 + 3 ) - 4".into()), 2);
    assert_eq!(Solution::calculate("5+3-4-(1+2-7+(10 - 1   + 3+5+(3-0+(8-(3+(8-(10-(6-10-8-7+(0+0+7)-10+5-3-2+(9+0+(7+(2-(2-(9)-2+5+4+2+(2+9+1+5+5-8-9-2-9+1+0)-(5-(9)-(0-(7+9)+(10+(6-4+6))+0-2+(10+7+(8+(7-(8-(3)+(2)+(10-6+10-(2)-7-(2)+(3+(8))+(1-3-8)+6-(4+1)+(6))+6-(1)-(10+(4)+(8)+(5+(0))+(3-(6))-(9)-(4)+(2))))))-1)))+(9+6)+(0))))+3-(1))+(7)))) ))))".into()), -35);
}
