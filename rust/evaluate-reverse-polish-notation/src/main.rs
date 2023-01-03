impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        tokens.into_iter().for_each(|x| {
            if let Ok(n) = x.parse() {
                stack.push(n);
            } else {
                let b = stack.pop().unwrap();
                let length = stack.len();
                let a = &mut stack[length - 1];
                match x.as_bytes()[0] {
                    b'+' => {
                        *a += b;
                    }
                    b'-' => {
                        *a -= b;
                    }
                    b'*' => {
                        *a *= b;
                    }
                    b'/' => {
                        *a /= b;
                    }
                    _ => {}
                }
            }
        });
        stack.pop().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
