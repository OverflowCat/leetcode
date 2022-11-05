use std::collections::{HashMap, HashSet};
struct Node {
    has_word: bool,
    next: HashMap<u8, Node>,
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Node {
            has_word: false,
            next: HashMap::with_capacity(4),
        };
        for x in words {
            let mut curr = &mut trie;
            let n = x.len() - 1;
            for (i, b) in x.into_bytes().into_iter().enumerate() {
                curr = curr
                    .next
                    .entry(b)
                    .and_modify(|node| node.has_word |= i == n)
                    // 注意这里 |=，如果写成 = 的话，已经是 true 了的就会被覆盖掉。见第二个测试样例。
                    .or_insert(Node {
                        has_word: i == n,
                        next: Default::default(),
                    });
            }
        }
        let m = board.len();
        let n = board[0].len();
        let mut res = HashSet::new();
        fn f(
            i: usize,
            j: usize,
            word: &mut String,
            board: &Vec<Vec<char>>,
            used: &mut Vec<Vec<bool>>,
            trie: &Node,
            res: &mut HashSet<String>,
        ) {
            if used[i][j] {
                return;
            }
            if let Some(x) = trie.next.get(&(board[i][j] as u8)) {
                used[i][j] = true;
                word.push(board[i][j]);
                if x.has_word {
                    res.insert(word.clone()); // 揾到
                }
                if i > 0 {
                    f(i - 1, j, word, board, used, x, res);
                }
                if i < board.len() - 1 {
                    f(i + 1, j, word, board, used, x, res);
                }
                if j > 0 {
                    f(i, j - 1, word, board, used, x, res);
                }
                if j < board[0].len() - 1 {
                    f(i, j + 1, word, board, used, x, res);
                }
                word.pop();
                used[i][j] = false;
            }
        }
        for i in 0..m {
            for j in 0..n {
                let mut used = vec![vec![false; n]; m];
                f(i, j, &mut String::new(), &board, &mut used, &trie, &mut res);
            }
        }
        res.into_iter().collect()
    }
}

struct Solution {}

fn main() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec!["oath".into(), "pea".into(), "eat".into(), "rain".into()];
    println!("{:?}", Solution::find_words(board, words));
    let board = vec![
        vec!['o', 'a', 'b', 'n'],
        vec!['o', 't', 'a', 'e'],
        vec!['a', 'h', 'k', 'r'],
        vec!['a', 'f', 'l', 'v'],
    ];
    let words = vec!["oa".into(), "oaa".into()];
    println!("{:?}", Solution::find_words(board, words));
}
