use std::collections::HashMap;
static DOT: u8 = '.' as u8;
static L: u8 = 'L' as u8;
static R: u8 = 'R' as u8;
static LR: u8 = L | R;

impl Solution {
    pub fn push_dominoes(mut dominoes: String) -> String {
        let last_i = dominoes.len() - 1;
        let mut events: HashMap<usize, u8> = HashMap::new();
        unsafe {
            let state = dominoes.as_mut_vec();
            state.iter().enumerate().for_each(|(i, &x)| {
                if x != DOT {
                    events.insert(i, x);
                }
            });
            while events.len() != 0 {
                let mut new_events: HashMap<usize, u8> = HashMap::new();
                events.into_iter().for_each(|(i, x)| match x {
                    _ if x == LR => {}
                    x => {
                        if state[i] == DOT {
                            state[i] = x
                        }
                        if x == L {
                            if i != 0 && state[i - 1] == DOT {
                                *new_events.entry(i - 1).or_insert(0) |= L;
                            }
                        } else if i < last_i && state[i + 1] == DOT {
                            *new_events.entry(i + 1).or_insert(0) |= R;
                        }
                    }
                });
                events = new_events;
            }
            String::from_utf8_unchecked(state.to_vec())
        }
    }
}